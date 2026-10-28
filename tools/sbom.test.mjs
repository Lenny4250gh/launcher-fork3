import { test } from 'node:test';
import assert from 'node:assert/strict';
import { normalize } from './sbom.mjs';

test('source identities and dependency edges survive without checkout locations', () => {
  const ref = 'path+file:///checkout/network#net@1.0.0';
  const registry = 'registry+https://example.invalid/index#dependency@2.0.0';
  const bom = { metadata: { component: { 'bom-ref': ref, name: 'net', version: '1.0.0', purl: 'pkg:cargo/net@1.0.0?download_url=file://.&executable=net#src/main.rs' } },
    components: [{ 'bom-ref': registry, name: 'dependency', version: '2.0.0' }],
    dependencies: [{ ref, dependsOn: [registry] }, { ref: registry, dependsOn: [] }] };
  const result = normalize(bom);
  assert.equal(result.metadata.component['bom-ref'], 'urn:anyplay:source:net%401.0.0');
  assert.equal(result.metadata.component.purl, 'pkg:cargo/net@1.0.0?executable=net#src/main.rs');
  assert.equal(result.dependencies[0].ref, result.metadata.component['bom-ref']);
  assert.equal(result.dependencies[0].dependsOn[0], registry);
  assert.deepEqual(result.components, bom.components);
  assert.ok(!JSON.stringify(result).includes('file://'));
  assert.deepEqual(normalize(result), result);
});

test('unresolved paths and ambiguous source identities refuse publication', () => {
  assert.throws(() => normalize({ dependencies: [{ ref: 'path+file:///checkout#missing' }] }));
  assert.throws(() => normalize({ components: [
    { 'bom-ref': 'path+file:///one#net', name: 'net', version: '1.0.0' },
    { 'bom-ref': 'path+file:///two#net', name: 'net', version: '1.0.0' },
  ] }));
});


test('a binary and a library with the same display name retain different package identities', () => {
  const worker = 'path+file:///checkout/worker#worker@1.0.0';
  const library = 'path+file:///checkout/share#share@1.0.0';
  const result = normalize({ metadata: { component: {
    'bom-ref': worker, name: 'share', version: '1.0.0', purl: 'pkg:cargo/worker@1.0.0',
  } }, components: [{ 'bom-ref': library, name: 'share', version: '1.0.0', purl: 'pkg:cargo/share@1.0.0' }],
  dependencies: [{ ref: worker, dependsOn: [library] }] });
  assert.notEqual(result.metadata.component['bom-ref'], result.components[0]['bom-ref']);
  assert.equal(result.dependencies[0].ref, result.metadata.component['bom-ref']);
  assert.equal(result.dependencies[0].dependsOn[0], result.components[0]['bom-ref']);
});

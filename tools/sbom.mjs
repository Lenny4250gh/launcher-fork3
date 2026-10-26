// Give source packages stable CycloneDX references without publishing checkout locations.
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

export function normalize(bom) {
  const references = new Map();
  const owners = new Map();
  function visit(value, callback) {
    if (Array.isArray(value)) value.forEach(item => visit(item, callback));
    else if (value && typeof value === 'object') {
      callback(value);
      Object.values(value).forEach(item => visit(item, callback));
    }
  }
  visit(bom, component => {
    const ref = component['bom-ref'];
    if (typeof ref !== 'string' || !ref.startsWith('path+file://')) return;
    if (!component.name || !component.version) throw new Error('Source component has no name or version');
    const stable = `urn:anyplay:source:${encodeURIComponent(component.name)}:${encodeURIComponent(component.version)}`;
    if (owners.has(stable) && owners.get(stable) !== ref) throw new Error('Ambiguous source component');
    owners.set(stable, ref);
    references.set(ref, stable);
    const purl = component.purl;
    if (typeof purl === 'string' && purl.includes('file://')) {
      const hash = purl.indexOf('#');
      const base = hash < 0 ? purl : purl.slice(0, hash);
      const subpath = hash < 0 ? '' : purl.slice(hash);
      const query = base.indexOf('?');
      const qualifiers = new URLSearchParams(query < 0 ? '' : base.slice(query + 1));
      if (qualifiers.get('download_url')?.startsWith('file://')) {
        qualifiers.delete('download_url');
        const rest = qualifiers.toString();
        references.set(purl, `${query < 0 ? base : base.slice(0, query)}${rest ? `?${rest}` : ''}${subpath}`);
      }
    }
  });
  function replace(value) {
    if (typeof value === 'string') {
      if (references.has(value)) return references.get(value);
      if (value.includes('file://')) throw new Error('Unresolved file location in SBOM');
      return value;
    }
    if (Array.isArray(value)) return value.map(replace);
    if (value && typeof value === 'object') {
      return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, replace(item)]));
    }
    return value;
  }
  return replace(bom);
}

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  if (process.argv.length < 3) throw new Error('Usage: node tools/sbom.mjs <SBOM files...>');
  for (const path of process.argv.slice(2)) {
    const bom = normalize(JSON.parse(await readFile(path, 'utf8')));
    await writeFile(path, `${JSON.stringify(bom, null, 2)}\n`);
  }
}

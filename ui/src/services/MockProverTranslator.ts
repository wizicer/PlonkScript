import { MockProverData } from './ConstraintSystem';

export function convertMockProverOutputToObject(
  content: string
): MockProverData {
  return JSON.parse(convertMockProverOutputToJson(content)) as MockProverData;
}

export function convertMockProverOutputToJson(content: string): string {
  // protect original string
  const pairs: Record<string, string> = {};
  // const reString = /".*[\{\}\[\]\(\)]+.*"/g;
  const reString = /".*"/g;
  const matches = content.matchAll(reString);
  for (const m of matches) {
    const rnd = `"${Math.random()}"`;
    pairs[rnd] = m[0];
    content = content.replaceAll(m[0], rnd);
  }

  // combine hash map of tuple and array
  content = content.replaceAll('): [', '');
  content = content.replaceAll(/^(\s*)\): (\d+),/gm, '$1    $2\n$1),');
  content = content.replaceAll(/\}: "(.*)",/g, '"value": "$1" }');

  // convert `columns` set to array
  content = content.replaceAll(
    /(columns: )\{(\n(?: {16}.*\n)+)( {12}})/g,
    '$1[$2            ]'
  );

  // convert `enabled_selectors` set to array
  content = content.replaceAll(
    /(enabled_selectors: )\{(\n(?: {16}.*\n)+)( {12}})/g,
    '$1[$2            ]'
  );

  // convert `cells` set to array
  content = content.replaceAll(
    /(cells: )\{(\n(?: {16}.*\n)+)( {12}})/g,
    '$1[$2            ]'
  );

  // convert `general_column_annotations` set to array
  content = content.replaceAll(
    /(general_column_annotations: )\{(\n(?: {12}.*\n)+)( {8}})/g,
    '$1[$2        ]'
  );

  // special case for Plonky2 data
  content = content.replaceAll(/_phantom: (PhantomData)<(.*)> \}\<.*\>,/g, '"_phantom": "$1" },');
  content = content.replaceAll(/([\w\d]+)\((PhantomData)<(.*)>\)<(.*)=(.*)>,/g, '{ "type": "$1", "$2": "$3", "$4": "$5" },');
  content = content.replaceAll(/_phantom: (PhantomData)<(.*)>/g, '"_phantom": "$1"');
  content = content.replaceAll(/([\w\d_]+)\s*{\s([\w\d_]+:\s*[\w\d\"_\']+,?\s)+}\s*\+\s*([\w\d_]+)\s*:\s*([\w\d_\"\']),/g, '$1 { $2,\n$3: $4 },');


  content = content.replaceAll(/({\s*)([^:]*)\s*\(/g, '$1"$2": (');

  // quote type
  content = content.replaceAll(/([\w\d]+) ?{/g, '{ type: "$1",');

  content = content.replaceAll(/([\w\d]+)\(/g, '[ "$1",');

  // quote range value
  content = content.replace(/(\d+\.\.\d+)/g, '"$1"');

  // quote special number like: 17049590034418533166 + 12587749074617431523*a,
  content = content.replace(/(\d+ \+ \d+\*a)/g, '"$1"');

  // convert brackets
  content = content.replaceAll(/\(/g, '[');
  content = content.replaceAll(/\)/g, ']');

  // standardize quoted key
  // content = content.replace(/(['"])?([\w\d]+)(['"])?:\s*/g, '"$2": ');
  content = content.replace(/[^"](['])?([\w\d]+)(['])?:[^:]\s*/g, '"$2": ');

  // quote value to string
  content = content.replace(/:\s*([^" {([\n]+),$/gm, ': "$1",');

  // remove trailing comma
  content = content.replace(/,(\s*[\}\]])/g, '$1');

  // hex to string
  content = content.replace(/(0x[\da-f]+)/g, '"$1"');

  // quote array item
  content = content.replace(/^(\s*)([^\n "([{}\]):,]+)(,?)$/gm, '$1"$2"$3');

  // quote prop key
  content = content.replace(/^(\s*)([\w\d]+):/gm, '$1"$2":');

  // restore protected string
  for (const key in pairs) {
    if (Object.prototype.hasOwnProperty.call(pairs, key)) {
      const val = pairs[key];
      content = content.replaceAll(key, val);
    }
  }

  return content;
}

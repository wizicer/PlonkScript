I have to write typescript program similar to my old code, here is for reference, my old code:

```
export type RotationType = ['Rotation', string];
export type PolynomialExpression =
  | ['Constant', string]
  // | ["Selector", ]
  | {
      type: 'Fixed' | 'Advice' | 'Instance';
      column_index: string;
      query_index: string;
      rotation: RotationType;
    }
  | ['Negated', PolynomialExpression]
  | ['Sum', PolynomialExpression, PolynomialExpression]
  | ['Product', PolynomialExpression, PolynomialExpression]
  | ['Scaled', PolynomialExpression, string];

export function stringifyGate(polys: PolynomialExpression): string {
  if (Array.isArray(polys)) {
    if (polys[0] == 'Constant') return BigInt(polys[1]).toString();
    if (polys[0] == 'Negated') {
      const inner = stringifyGate(polys[1]);
      if (
        inner.indexOf('+') >= 0 ||
        inner.indexOf('-') >= 0 ||
        inner.indexOf('*') >= 0
      ) {
        return ` - (${inner})`;
      }
      return ` - ${inner}`;
    }

    if (polys[0] == 'Sum') {
      const second = stringifyGate(polys[2]);
      return `${stringifyGate(polys[1])}${
        second.startsWith(' -') ? '' : ' + '
      }${second}`;
    }
    if (polys[0] == 'Product') {
      return `${quoteIfIncludeAddSub(
        stringifyGate(polys[1])
      )} * ${quoteIfIncludeAddSub(stringifyGate(polys[2]))}`;
    }
    if (polys[0] == 'Scaled')
      return `${quoteIfIncludeAddSub(
        stringifyGate(polys[1])
      )} * ${quoteIfIncludeAddSub(shortenGateValue(polys[2]))}`;
    if (polys[0] == 'SelectorExpression')
      // special type from tiny-ram-halo2
      return `{${stringifyGate(polys[1])}}`;
  }

  // console.log('object polys', polys);

  if (!polys.rotation) console.warn('wrong rotation', polys);
  //TODO: standardize column name getting
  const rotationHint = polys.rotation[1] == '0' ? '' : `[${polys.rotation[1]}]`;
  return `${polys.type[0].toLowerCase()}_${polys.column_index}${rotationHint}`;
}
```

here is the new nested definition for Expression
```
export type SymbolicExpression<F> =
  | { type: 'Variable'; entry: Entry; index: number }
  | { type: 'IsFirstRow' }
  | { type: 'IsLastRow' }
  | { type: 'IsTransition' }
  | { type: 'Constant'; value: F }
  | {
      type: 'Add';
      x: SymbolicExpression<F>;
      y: SymbolicExpression<F>;
      degree_multiple: number;
    }
  | {
      type: 'Sub';
      x: SymbolicExpression<F>;
      y: SymbolicExpression<F>;
      degree_multiple: number;
    }
  | { type: 'Neg'; x: SymbolicExpression<F>; degree_multiple: number }
  | {
      type: 'Mul';
      x: SymbolicExpression<F>;
      y: SymbolicExpression<F>;
      degree_multiple: number;
    };

export type Entry =
  | { type: 'Preprocessed'; offset: number }
  | { type: 'Main'; offset: number }
  | { type: 'Permutation'; offset: number }
  | { type: 'Public' }
  | { type: 'Challenge' };
```
PS: quoteIfIncludeAddSub and shortenGateValue is the function that beautify the generated string, keep them as just that.

the expected function signature looks like this:
```
export function stringifyGate<F>(polys: SymbolicExpression<F>): string
```
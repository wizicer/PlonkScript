export interface IDataModel {
  name: string;
  data: Plonky3Data;
  title?: string;
  description?: string;
  sourceUrl?: string;
}

export const dataList: IDataModel[] = [];

export type Num = number;
export type Plonky3Data = All<Num>;

interface All<F> {
  symbols: SymbolicExpression<F>[];
  trace: DenseMatrix<F>;
  public: Array<F>
}

export interface DenseMatrix<F> {
  values: F[];
  width: number;
}

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

// =======================

export interface RowField {
  index: number;
  value: Num;
  row: number;
  col: number;
}

export type WholeRow = { index: number } & Record<number, RowField>;

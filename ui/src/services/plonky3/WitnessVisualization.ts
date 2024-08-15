import { QTableColumn } from 'quasar';
import { quoteIfIncludeAddSub } from '../ConstraintSystem';
import { SymbolicExpression, WholeRow, Entry } from './DefaultModels';

export function getColumns(cols: number): QTableColumn[] {
  const columns: QTableColumn[] = [];
  columns.push({
    name: 'index',
    label: 'idx',
    align: 'center',
    field: 'index',
    sortable: false,
  });

  for (let i = 0; i < cols; i++) {
    columns.push({
      name: `${i}`,
      label: `a_${i}`,
      align: 'center',
      field: `${i}`,
      sortable: false,
    });
  }

  return columns;
}

export function getRows(witness: number[], width: number): WholeRow[] {
  const rows: WholeRow[] = [];
  const colnum = width;
  const total = witness.length;
  const rownum = total / colnum;

  // witness
  for (let r = 0; r < rownum; r++) {
    const row: WholeRow = { index: r };
    for (let c = 0; c < colnum; c++) {
      const idx = r * colnum + c;
      row[c] = {
        value: witness[idx],
        index: r,
        row: r,
        col: c,
      };
    }

    rows.push(row);
  }

  return rows;
}

export function stringifySymExp<F>(sexp: SymbolicExpression<F>): string {
  switch (sexp.type) {
    case 'Variable': {
      return stringifyEntry(sexp.entry, sexp.index);
    }
    case 'IsFirstRow':
      return 'IsFirstRow';
    case 'IsLastRow':
      return 'IsLastRow';
    case 'IsTransition':
      return 'IsTransition';
    case 'Constant':
      return (sexp.value as any).toString();
    case 'Add': {
      const xStr = stringifySymExp(sexp.x);
      const yStr = stringifySymExp(sexp.y);
      return `${xStr} + ${yStr}`;
    }
    case 'Sub': {
      const xStr = stringifySymExp(sexp.x);
      const yStr = quoteIfIncludeAddSub(stringifySymExp(sexp.y));
      return `${xStr} - ${yStr}`;
    }
    case 'Neg': {
      const xStr = quoteIfIncludeAddSub(stringifySymExp(sexp.x));
      return ` - ${xStr}`;
    }
    case 'Mul': {
      const xStr = quoteIfIncludeAddSub(stringifySymExp(sexp.x));
      const yStr = quoteIfIncludeAddSub(stringifySymExp(sexp.y));
      return `${xStr} * ${yStr}`;
    }
    default:
      throw new Error(`Unknown SymbolicExpression type: ${sexp}`);
  }
}

function stringifyEntry(entry: Entry, index: number): string {
  function getOffsetSuffix(offset: number): string {
    if (offset === 0) {
      return '';
    } else {
      return `[${offset}]`;
    }
  }
  switch (entry.type) {
    case 'Preprocessed':
      return `f_${index}${getOffsetSuffix(entry.offset)}`;
    case 'Main':
      return `a_${index}${getOffsetSuffix(entry.offset)}`;
    case 'Permutation':
      return `p_${index}${getOffsetSuffix(entry.offset)}`;
    case 'Public':
      return 'public';
    case 'Challenge':
      return 'challenge';
    default:
      throw new Error(`Unknown Entry type: ${entry}`);
  }
}

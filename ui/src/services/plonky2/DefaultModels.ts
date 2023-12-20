export interface IDataModel {
  name: string;
  data: Plonky2Data;
  title?: string;
  description?: string;
  sourceUrl?: string;
}

type Num = string;
type ExtensionFieldNum = string;
type RangeString = string;
type Bool = 'true' | 'false';

export const dataList: IDataModel[] = [];

export interface Plonky2Data {
  type: 'Plonky2Data';
  witness: Witness;
  data: CircuitData;
  proof?: Plonky2Proof;
}

export interface CircuitData {
  type: 'CircuitData';
  prover_only: ProverOnly;
  verifier_only: VerifierOnly;
  common: CommonCircuitData;
}

export interface CommonCircuitData {
  type: 'CommonCircuitData';
  config: CommonConfig;
  fri_params: FriParams;
  gates: GateElement[];
  selectors_info: SelectorsInfo;
  quotient_degree_factor: Num;
  num_gate_constraints: Num;
  num_constants: Num;
  num_public_inputs: Num;
  k_is: Num[];
  num_partial_products: Num;
  num_lookup_polys: Num;
  num_lookup_selectors: Num;
  luts: [];
}

export interface CommonConfig {
  type: 'CircuitConfig';
  num_wires: Num;
  num_routed_wires: Num;
  num_constants: Num;
  use_base_arithmetic_gate: Bool;
  security_bits: Num;
  num_challenges: Num;
  zero_knowledge: Bool;
  max_quotient_degree_factor: Num;
  fri_config: FriConfig;
}

export interface FriConfig {
  type: 'FriConfig';
  rate_bits: Num;
  cap_height: Num;
  proof_of_work_bits: Num;
  reduction_strategy: string[];
  num_query_rounds: Num;
}

export interface FriParams {
  type: 'FriParams';
  config: FriConfig;
  hiding: Bool;
  degree_bits: Num;
  reduction_arity_bits: [];
}

export type GateElement = GateClass | string;

export interface GateClass {
  type: string;
  num_consts?: number;
  num_ops?: number;
  PhantomData?: string;
  WIDTH?: Num;
}

export interface SelectorsInfo {
  type: 'SelectorsInfo';
  selector_indices: Num[];
  groups: RangeString[];
}

export interface ProverOnly {
  type: 'ProverOnlyCircuitData';
  generators: Generator[];
  generator_indices_by_watches: { [key: Num]: Num[] };
  constants_sigmas_commitment: PolynomialBatch;
  sigmas: Array<Num[]>;
  subgroup: Num[];
  public_inputs: Target[];
  representative_map: Num[];
  fft_root_table: FftRootTable;
  circuit_digest: HashOut;
  lookup_rows: [];
  lut_to_lookups: [];
}

export interface HashOut {
  type: 'HashOut';
  elements: Num[];
}

export interface PolynomialBatch {
  type: 'PolynomialBatch';
  polynomials: PolynomialCoeffs[];
  merkle_tree: MerkleTree;
  degree_log: Num;
  rate_bits: Num;
  blinding: Bool;
}

export interface MerkleTree {
  type: 'MerkleTree';
  leaves: Array<Num[]>;
  digests: HashOut[];
  cap: Cap;
}

export type Cap = ['MerkleCap', HashOut[]];

export interface PolynomialCoeffs {
  type: 'PolynomialCoeffs';
  coeffs: Num[];
}

export type FftRootTable = ['Some', Array<string[]>];

export type Generator =
  | 'RandomValueGenerator'
  | 'ConstantGenerator'
  | 'ArithmeticBaseGenerator'
  | 'PoseidonGenerator';

export type Target = TargetWire | VirtualTarget;

export interface TargetWire {
  type: 'Wire';
  row: Num;
  column: Num;
}

export interface VirtualTarget {
  type: 'VirtualTarget';
  index: Num;
}

export interface VerifierOnly {
  type: string;
  constants_sigmas_cap: Cap;
  circuit_digest: HashOut;
}

export interface Plonky2Proof {
  type: 'ProofWithPublicInputs';
  proof: Proof;
  public_inputs: Num[];
}

export interface Proof {
  type: 'Proof';
  wires_cap: Cap;
  plonk_zs_partial_products_cap: Cap;
  quotient_polys_cap: Cap;
  openings: OpeningSet;
  opening_proof: FriProof;
}

export interface FriProof {
  type: 'FriProof';
  commit_phase_merkle_caps: [];
  query_round_proofs: FriQueryRound[];
  final_poly: PolynomialCoeffs;
  pow_witness: Num;
}

export interface FriQueryRound {
  type: 'FriQueryRound';
  initial_trees_proof: FriInitialTreeProof;
  steps: [];
}

export interface FriInitialTreeProof {
  type: 'FriInitialTreeProof';
  evals_proofs: Array<[Num[], EvalsProofClass]>;
}

export interface EvalsProofClass {
  type: 'MerkleProof';
  siblings: HashOut[];
}

export interface OpeningSet {
  type: 'OpeningSet';
  constants: ExtensionFieldNum[];
  plonk_sigmas: ExtensionFieldNum[];
  wires: ExtensionFieldNum[];
  plonk_zs: ExtensionFieldNum[];
  plonk_zs_next: ExtensionFieldNum[];
  partial_products: ExtensionFieldNum[];
  quotient_polys: ExtensionFieldNum[];
  lookup_zs: [];
  lookup_zs_next: [];
}

export interface Witness {
  type: 'PartitionWitness';
  values: WitnessValue[];
  representative_map: Num[];
  num_wires: Num;
  degree: Num;
}

export type WitnessValue = 'None' | ['Some', Num];

export interface RowField {
  raw?: string;
  value: WitnessValue;
}
export interface RowFieldWithPosition extends RowField {
  index: number;
  raw_index: number;
  representative_map: number;
  representative_value: WitnessValue;
  row: number;
  col: number;
}

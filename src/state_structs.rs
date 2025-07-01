/*
State struct - taken from the julia version
Will need to check for the sparse type if it becomes necessay
Not much support so we will use simple matrices
*/


pub struct GeneState {
    n_genes: i64,
    genes: Vec<String>,
    saturation_rank: Vec<i64>,
    leaky_rank: Vec<i64>,
    decay_rate: Vec<f64>,
    translation_efficiency: Vec<f64>,
    grn_state: Vec<Vec<i32>>,
}


impl GeneState {
    pub fn new(
        n_genes:i64,
        genes:Vec<String>,
        saturation_rank: Vec <i64>,
        leaky_rank: Vec<i64>,
        decay_rate: Vec<f64>,
        translation_efficiency: Vec<f64>,
        grn_state: Vec<Vec<i32>>) -> Self {
        GeneState{
            n_genes,
            genes,
            saturation_rank,
            leaky_rank,
            decay_rate,
            translation_efficiency,
            grn_state}
    }
}

/*
GRN Struct 
Temporary struct since I am not sure how I will encode complex GRNs
*/

pub struct Grn {
    master_regulator: Vec<i64>,
    regulation: Vec<Vec<i32>>,
    chromatin_remodelling: Vec<i64>,
    tf_binding: Vec<i64>,
}

impl Grn {
    pub fn new(
        master_regulator: Vec<i64>,
        regulation: Vec<Vec<i32>>,
        chromatin_remodelling: Vec<i64>,
        tf_binding: Vec<i64>) -> Self {
        Grn {
            master_regulator,
            regulation,
            chromatin_remodelling,
            tf_binding
        }
    }
}


/*
Cell State struct
All cells will have a single CellState struct
*/


pub struct CellState {
    temporal_state: i64,
    ecosystem: Vec<Vec<f64>>,
    coordinates: Vec<f64>,
    chromatin_state: Vec<f64>,
    binding_state: Vec<f64>,
    rna_state: Vec<i64>,
    protein_state: Vec<i64>,
    meta_state: Vec<Vec<i64>>,
    wrap_state: Vec<Vec<i64>>,
    trajectory: Vec<String>,
    grn_collection: Vec<Vec<i64>>,
    cell_type: String,
    domain: String,
}

impl CellState {
    pub fn new(
        temporal_state: i64,
        ecosystem: Vec<Vec<f64>>,
        coordinates: Vec<f64>,
        chromatin_state: Vec<f64>,
        binding_state: Vec<f64>,
        rna_state: Vec<i64>,
        protein_state: Vec<i64>,
        meta_state: Vec<Vec<i64>>,
        wrap_state: Vec<Vec<i64>>,
        trajectory: Vec<String>,
        grn_collection: Vec<Vec<i64>>,
        cell_type: String,
        domain: String) -> Self {
        CellState {
            temporal_state,
            ecosystem,
            coordinates,
            chromatin_state,
            binding_state,
            rna_state,
            protein_state,
            meta_state,
            wrap_state,
            trajectory,
            grn_collection,
            cell_type,
            domain
        }
    }
}

/*
Tissue state 
Will be mutable but i guess this doesn't matter here when writing 
the struct definition 
*/

pub struct TissueState {
    n_cells: i64,
    coordinates: Vec<Vec<f64>>,
    cell_types: Vec<String>,
    territories: Vec<String>,
    cell_distance: Vec<Vec<f64>>,
    max_diffusion: f64,
    density_damp: f64,
    diffusion_damp: f64,
    static_cells: bool,
}

impl TissueState{
    pub fn new(
        n_cells: i64,
        coordinates: Vec<Vec<f64>>,
        cell_types: Vec<String>,
        territories: Vec<String>,
        cell_distance: Vec<Vec<f64>>,
        max_diffusion: f64,
        density_damp: f64,
        diffusion_damp: f64,
        static_cells: bool) -> Self {
        TissueState {
            n_cells,
            coordinates,
            cell_types,
            territories,
            cell_distance,
            max_diffusion,
            density_damp,
            diffusion_damp,
            static_cells
        }
    }
}



/*
Temporal State
Keeping track of temporal state - rust might have better disk IO
*/
pub struct TemporalState {
    total_steps: i64,
    current_step: i64,
    sample_at: Vec<i64>,
    past_tissue_state: Vec<TissueState>,
    current_tissue_state: TissueState,
    past_cell_state: Vec<Vec<CellState>>,
    current_cell_state: Vec<CellState>,
    save_state: bool,
}

impl TemporalState {
    pub fn new(
        total_steps: i64,
        current_step: i64,
        sample_at: Vec<i64>,
        past_tissue_state: Vec<TissueState>,
        current_tissue_state: TissueState,
        past_cell_state: Vec<Vec<CellState>>,
        current_cell_state: Vec<CellState>,
        save_state: bool) -> Self {
        TemporalState {
            total_steps,
            current_step,
            sample_at,
            past_tissue_state,
            current_tissue_state,
            past_cell_state,
            current_cell_state,
            save_state
        }
    }
}

/*
Sample State
*/
pub struct SampleState {
    n_cells: i64,
    n_genes: i64,
    batch: i64,
    tissue: TissueState,
    cells: Vec<CellState>,
    temporal_state: TemporalState,
    biological_out: Vec<String>,
    out: Vec<Vec<f64>>,
}

impl SampleState {
    pub fn new(
        n_cells: i64,
        n_genes: i64,
        batch: i64,
        tissue: TissueState,
        cells: Vec<CellState>,
        temporal_state: TemporalState,
        biological_out: Vec<String>,
        out: Vec<Vec<f64>>) -> Self {
        SampleState {
            n_cells,
            n_genes,
            batch,
            tissue,
            cells,
            temporal_state,
            biological_out,
            out
        }
    }
}


/*
Atlas State
Essentially high level parameters passing and storing of samples
Using a new trait instead of default for now

Will need to implement it - just don't get it for now

*/

pub struct AtlasState {
    pub cells_per_sample: Vec<i64>,
    pub genes_per_sample: Vec<i64>,
    pub n_samples: i64,
    pub n_batches: i64,
    pub domain_type: Vec<String>,
    pub initial_cell_type: i64,
    pub grn_type: Vec<String>,
    pub biological_out: Vec<String>,
    pub mosaic: bool,
    pub biological_noise: i64,
    pub technical_noise: i64,
    pub save_sample: bool,
    samples: Option<Vec<SampleState>>,
}

impl AtlasState {
    pub fn new() -> Self {
        AtlasState {
            cells_per_sample: [1000, 10_000].to_vec(),
            genes_per_sample: [100, 10_000].to_vec(),
            n_samples: 1,
            n_batches: 1,
            domain_type: vec!["circle".to_string()],
            initial_cell_type: 10,
            grn_type: vec!["reprisilator".to_string(), "random_template".to_string()],
            biological_out: vec!["rna".to_string()],
            mosaic: false,
            biological_noise: 10,
            technical_noise: 10,
            save_sample: true,
            samples: None
        }
    }
}





/*
BioRef if we allow the use of biological references
*/
pub struct BioRef{
    coordinates: Vec<f64>,
    ref_type: String,
    biological_measurement: Vec<Vec<f64>>,
    cell_type_labels: Vec<String>,
    domain_labels: Vec<String>,
}

impl BioRef {
    pub fn new(
        coordinates: Vec<f64>,
        ref_type: String,
        biological_measurement: Vec<Vec<f64>>,
        cell_type_labels: Vec<String>,
        domain_labels: Vec<String>) -> Self {
        BioRef {
            coordinates,
            ref_type,
            biological_measurement,
            cell_type_labels,
            domain_labels
        }
    }
}



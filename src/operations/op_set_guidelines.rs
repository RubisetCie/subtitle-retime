use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Gap modes enumeration
 */
pub enum OpSetGuidelinesPreset {
    Netflix,
    Ted,
    BBC
}

fn preset_to_string(preset: &OpSetGuidelinesPreset) -> &str {
    match preset {
        OpSetGuidelinesPreset::Netflix   => "Netflix",
        OpSetGuidelinesPreset::Ted   => "Ted",
        OpSetGuidelinesPreset::BBC   => "BBC"
    }
}

/*
 * Setting guidelines preset structure
 */
pub struct OpSetGuidelines {
    preset: OpSetGuidelinesPreset
}

/*
 * Argument identifier
 */
impl OpSetGuidelines {
    pub fn new(preset: OpSetGuidelinesPreset) -> Self {
        Self { preset }
    }
}

/*
 * Setting guidelines preset implementation
 */
impl Operation for OpSetGuidelines {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting validation guidelines to: {}", preset_to_string(&self.preset)); }
        match self.preset {
            OpSetGuidelinesPreset::Netflix => {
                st.max_cps   = 20.0;
                st.max_cpl   = 42;
                st.max_lines = 2;
                st.max_dur   = 7000;
                st.min_dur   = 833;
                st.min_gap   = 83;
            },
            OpSetGuidelinesPreset::Ted => {
                st.max_cps   = 21.0;
                st.max_cpl   = 42;
                st.max_lines = 2;
                st.max_dur   = u64::MAX;
                st.min_dur   = u64::MIN;
                st.min_gap   = u64::MIN;
            },
            OpSetGuidelinesPreset::BBC => {
                st.max_cps   = 17.0;
                st.max_cpl   = 37;
                st.max_lines = 2;
                st.max_dur   = 7000;
                st.min_dur   = 1000;
                st.min_gap   = u64::MIN;
            }
        }
    }
}

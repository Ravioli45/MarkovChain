use rand::distributions::{Uniform, Distribution};
use rand::rngs::ThreadRng;

pub struct MarkovChain{

    rng: ThreadRng,
    u_dist: Uniform<f64>,
    state_probs: Vec<Vec<f64>>,
    state_labels: Vec<String>,
    state: usize
    
}
impl MarkovChain{
    pub fn new(probabilities: Vec<Vec<f64>>, states: Vec<String>) -> MarkovChain{

        for p in &probabilities{
            if p.iter().sum::<f64>() != 1.0{
                panic!("Probabilities do not add to one");
            }
        }
        if states.len() != probabilities.len(){
            panic!("Mismatched number of states and labels");
        }
        
        //TODO check validity of probability vectors
        //TODO check for correct number of labels
        let c = MarkovChain{
            rng: rand::thread_rng(),
            u_dist: Uniform::from(0.0..1.0),
            state_probs: probabilities,
            state_labels: states,
            state: 0
        };
        
        c
    }

    fn random(&mut self) -> f64{
        //returns random value f32 between 0.0 and 1.0
        //based on uniform distribution
        self.u_dist.sample(&mut self.rng)
    }
    
    pub fn current_state(&self) -> &String{
        &self.state_labels[self.state]
    }
    
    pub fn next_state(&mut self) -> &String{

        let r: f64 = self.random();
        let current_prob: &Vec<f64> = &(self.state_probs[self.state]);
        //let r: f64 = self.random();

        //let mut i: usize = 0;
        let mut lower_bound: f64 = 0.0;
        let mut upper_bound: f64 = 0.0;

        for i in 0..current_prob.len(){
            
            
            if i >= 1{
                lower_bound += current_prob[i-1];
            }
            upper_bound += current_prob[i];

            if lower_bound <= r && r < upper_bound {
                self.state = i;
                break;
            }
            
        }
        
        &self.state_labels[self.state]
    }
}

use rsrl::{
    control::td::QLearning,
    domains::{Domain, Observation, Reward},
    fa::linear::{
        basis::{Combinators, Fourier},
        optim::SGD,
        LFA,
    },
    make_shared,
    policies::{Greedy, Policy},
    spaces::{discrete::Ordinal, real::Interval, ProductSpace, Space},
    Handler,
};

use rand::{rngs::StdRng, SeedableRng};

const X_MIN: f64 = -1.2;
const X_MAX: f64 = 0.6;

const V_MIN: f64 = -0.07;
const V_MAX: f64 = 0.07;

const FORCE_G: f64 = -0.0025;
const FORCE_CAR: f64 = 0.001;

const HILL_FREQ: f64 = 3.0;

const REWARD_STEP: f64 = -1.0;
const REWARD_GOAL: f64 = 0.0;

const ALL_ACTIONS: [f64; 3] = [-1.0, 0.0, 1.0];

pub struct MountainCar1 {
    x: f64,
    v: f64,
}

impl MountainCar1 {
    pub fn new(x: f64, v: f64) -> MountainCar1 {
        MountainCar1 { x, v }
    }

    fn dv(x: f64, a: f64) -> f64 {
        FORCE_CAR * a + FORCE_G * (HILL_FREQ * x).cos()
    }

    fn update_state(&mut self, a: usize) {
        let a = ALL_ACTIONS[a];

        self.v = clip!(V_MIN, self.v + Self::dv(self.x, a), V_MAX);
        self.x = clip!(X_MIN, self.x + self.v, X_MAX);
    }
}

impl Default for MountainCar1 {
    fn default() -> MountainCar1 {
        MountainCar1::new(-0.5, 0.0)
    }
}

impl Domain for MountainCar1 {
    type StateSpace = ProductSpace<Interval>;
    type ActionSpace = Ordinal;

    fn emit(&self) -> Observation<Vec<f64>> {
        if self.x >= X_MAX {
            Observation::Terminal(vec![self.x, self.v])
        } else {
            Observation::Full(vec![self.x, self.v])
        }
    }

    fn step(&mut self, action: &usize) -> (Observation<Vec<f64>>, Reward) {
        self.update_state(*action);

        let to = self.emit();
        let reward = if to.is_terminal() {
            REWARD_GOAL
        } else {
            REWARD_STEP
        };

        (to, reward)
    }

    fn state_space(&self) -> Self::StateSpace {
        ProductSpace::empty() + Interval::bounded(X_MIN, X_MAX) + Interval::bounded(V_MIN, V_MAX)
    }

    fn action_space(&self) -> Ordinal {
        Ordinal::new(3)
    }
}

pub fn build() {
    let env = MountainCar1::default();
    let n_actions = env.action_space().card().into();

    let mut rng = StdRng::seed_from_u64(0);
    let (mut ql, policy) = {
        let basis = Fourier::from_space(5, env.state_space()).with_bias();
        let q_func = make_shared(LFA::vector(basis, SGD(0.001), n_actions));
        let policy = Greedy::new(q_func.clone());

        (QLearning { q_func, gamma: 0.9 }, policy)
    };

    for e in 0..200 {
        // Episode loop:
        let mut j = 0;
        let mut env = MountainCar1::default();
        let mut action = policy.sample(&mut rng, env.emit().state());

        for i in 0.. {
            // Trajectory loop:
            j = i;

            let t = env.transition(action);

            ql.handle(&t).ok();
            action = policy.sample(&mut rng, t.to.state());

            if t.terminated() {
                break;
            }
        }

        println!("Batch {}: {} steps...", e + 1, j + 1);
    }

    let traj = MountainCar1::default().rollout(|s| policy.mode(s), Some(500));

    println!("OOS: {} states...", traj.n_states());
}

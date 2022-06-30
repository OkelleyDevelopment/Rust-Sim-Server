# Lotka Volterra Predator and Prey Simulation

Simple Predator-Prey Simulation API

**URL**: `/pred_prey`
**METHOD**: `POST`
**AUTH required**: NO

**DATA**:

```
{
    max_time: req_body.max_time,
    prey_pop: req_body.prey_pop,
    pred_pop: req_body.pred_pop,
    alpha: req_body.alpha,
    beta: req_body.beta,
    gamma: req_body.gamma,
    delta: req_body.delta,
}
```

**Fields**:

- `max_time`: max amount time steps
- `prey_pop`: starting prey population
- `pred_pop`: starting predator population
- `alpha`: growth rate of the prey
- `beta`: death rate of the prey due to the predators
- `gamma`: natural death rate of the predators
- `delta`: factor describing how many consumed prey create new predators

## Success Response

**Code**: `200 Ok`

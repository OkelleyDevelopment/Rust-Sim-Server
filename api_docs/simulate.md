# Simulate

Example Simulation API

**URL**: `/simulate/`
**METHOD**: `POST`
**AUTH required**: NO

**DATA**:

```
{
    timestep: i32,
    }
```

**Fields**:

- `timestep`: number to be subtracted from

## Success Response

**Code**: `200 Ok`

Will receive a JSON object with the updated time step count.

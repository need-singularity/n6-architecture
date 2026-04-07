"""PID controller bank for closed-loop consciousness state control."""

from brainwire.variables import VAR_NAMES


class PIDController:
    def __init__(self, Kp: float = 1.0, Ki: float = 0.1, Kd: float = 0.01,
                 max_integral: float = 10.0, output_limit: float = 5.0):
        self.Kp = Kp
        self.Ki = Ki
        self.Kd = Kd
        self.max_integral = max_integral
        self.output_limit = output_limit
        self._integral = 0.0
        self._prev_error = 0.0

    def update(self, setpoint: float, measured: float, dt: float) -> float:
        error = setpoint - measured
        self._integral += error * dt
        self._integral = max(-self.max_integral, min(self.max_integral, self._integral))
        derivative = (error - self._prev_error) / dt if dt > 0 else 0.0
        self._prev_error = error
        output = self.Kp * error + self.Ki * self._integral + self.Kd * derivative
        return max(-self.output_limit, min(self.output_limit, output))

    def reset(self):
        self._integral = 0.0
        self._prev_error = 0.0


class PIDBank:
    def __init__(self, default_Kp: float = 1.0, default_Ki: float = 0.1, default_Kd: float = 0.01):
        self.controllers: dict[str, PIDController] = {}
        for var in VAR_NAMES:
            self.controllers[var] = PIDController(Kp=default_Kp, Ki=default_Ki, Kd=default_Kd)

    def update(self, target: dict[str, float], measured: dict[str, float], dt: float) -> dict[str, float]:
        outputs = {}
        for var in VAR_NAMES:
            outputs[var] = self.controllers[var].update(setpoint=target[var], measured=measured[var], dt=dt)
        return outputs

    def apply_hints(self, hints: list) -> None:
        for hint in hints:
            if hint.var in self.controllers:
                c = self.controllers[hint.var]
                c.Kp *= hint.Kp_scale
                c.Ki *= hint.Ki_scale
                c.Kd *= hint.Kd_scale

    def reset_all(self):
        for c in self.controllers.values():
            c.reset()

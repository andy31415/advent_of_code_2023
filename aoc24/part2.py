#!/usr/bin/env python
import sympy
import sys

class H:
    def __init__(self, s, d):
        self.s = s
        self.d = d
        
    def __repr__(self):
        return f"HS({self.s} + t*{self.d})"

class Symbols:
    def __init__(self):
        self.s = {}
        self._h = []
        for name in [f"sx", f"sy", f"sz", f"dx", f"dy", f"dz" ]:
           self.s[name] = sympy.symbols(name)

    def add_all(self, h):
        for idx, x in enumerate(h):
            self._h.append(x)
            self._add_for_h(x, idx)

    def _add_for_h(self, h, idx):
        self.s[f"t{idx}"] = sympy.symbols(f"t{idx}")
           
    def solve(self):
        equations = []
        
        for i, h in enumerate(self._h):
            # s1 + v1*t1 = s + v * t1
            for vi, p in enumerate("xyz"):
                equations.append(
                        h.s[vi] + h.d[vi]*self.s[f"t{i}"] 
                        - self.s[f"s{p}"]
                        - self.s[f"d{p}"]*self.s[f"t{i}"])

        for e in equations:
           print(e)

        v = [x for x in self.s.values()]
        return sympy.solve(equations, v)[0]

        # Build every single equiation we know of
        


data = []
with open(sys.argv[1], "rt") as f:
    for l in f.readlines():
        s, d = l.split(" @ ")
        
        s = [x for x in map(float, s.split(", "))][:]
        d = [x for x in map(float, d.split(", "))][:]
        data.append(H(s,d))
        
for h in data:
   print(h)

# figure out times and symbolics... all possible equations
s = Symbols()
s.add_all(data)
    
print("SYMBOLS ADDED: ", s.s)
result = s.solve()

print("Place: ", result[0:6])
print("SUM OF POSITION", result[0] + result[1] + result[2])

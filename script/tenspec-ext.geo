vs() = Volume{:};

bb = newv;
Box(newv) = {-var_r1-var_r3,-var_r2,-var_l1-var_le-var_l2-var_r3, 2*(var_r1+var_r3),2*var_r2,2*(var_l1+var_le+var_l2+var_r3)};

cb1 = newv;
Box(newv) = {-var_r1-var_r3,-var_r2,-var_l1-var_le, var_r3,2*var_r2,2*(var_l1+var_le)};
cb2 = newv;
Box(newv) = {var_r1,-var_r2,-var_l1-var_le, var_r3,2*var_r2,2*(var_l1+var_le)};

cc1 = newv;
Cylinder(newv) = {-var_r1-var_r3,-var_r2,-var_l1-var_le, 0,2*var_r2,0, var_r3};
cc2 = newv;
Cylinder(newv) = {-var_r1-var_r3,-var_r2,var_l1+var_le, 0,2*var_r2,0, var_r3};
cc3 = newv;
Cylinder(newv) = {var_r1+var_r3,-var_r2,-var_l1-var_le, 0,2*var_r2,0, var_r3};
cc4 = newv;
Cylinder(newv) = {var_r1+var_r3,-var_r2,var_l1+var_le, 0,2*var_r2,0, var_r3};

mono = BooleanDifference { Volume{bb}; Delete; }{ Volume{cb1,cb2,cc1,cc2,cc3,cc4}; Delete; };
BooleanFragments { Volume{mono}; Delete; }{ Volume{vs()}; Delete; }

Mesh.CharacteristicLengthMin = var_cl;
Mesh.CharacteristicLengthMax = var_cl;
Mesh.CharacteristicLengthFromPoints = 0;
Mesh.CharacteristicLengthFromCurvature = 0;
Mesh.CharacteristicLengthExtendFromBoundary = 0;

Mesh.Optimize = 1;
Mesh.OptimizeNetgen = 1;
Mesh 3;
OptimizeMesh "Gmsh";

Plugin(AnalyseMeshQuality).IGEMeasure = 1;
Plugin(AnalyseMeshQuality).DimensionOfElements = 3;
Plugin(AnalyseMeshQuality).Run;

Mesh.MshFileVersion = 2;
Mesh.SaveAll = 1;

# Geometry (Rust)

Each file is a standalone program: `rustc --edition 2021 <file>.rs && ./<file>` runs its asserts.

## LeetCode
| File | Description |
|---|---|
| 0149_max_points_on_a_line.rs | Most points on one line via normalized slope hashing |
| 0223_rectangle_area.rs | Union area of two axis-aligned rectangles |
| 0356_line_reflection.rs | Vertical mirror axis check for a point set |
| 0391_perfect_rectangle.rs | Exact rectangle cover via area and corner parity |
| 0447_number_of_boomerangs.rs | Count equidistant ordered triples |
| 0469_convex_polygon.rs | Convexity test with consistent cross-product sign |
| 0587_erect_the_fence.rs | Convex hull that keeps collinear boundary points (monotone chain) |
| 0593_valid_square.rs | Square check from sorted pairwise distances |
| 0812_largest_triangle_area.rs | Largest triangle over all triples |
| 0836_rectangle_overlap.rs | Positive-area overlap of two rectangles |
| 0850_rectangle_area_ii.rs | Union area of many rectangles via coordinate compression |
| 0858_mirror_reflection.rs | Laser receptor via unfolding and lcm parity |
| 0892_surface_area_of_3d_shapes.rs | Surface area of stacked cube towers |
| 0939_minimum_area_rectangle.rs | Smallest axis-aligned rectangle from point set |
| 0963_minimum_area_rectangle_ii.rs | Smallest rotated rectangle via diagonal grouping |
| 0973_k_closest_points_to_origin.rs | K nearest points with quickselect |
| 1037_valid_boomerang.rs | Three distinct non-collinear points |
| 1232_check_if_it_is_a_straight_line.rs | Collinearity of all points |
| 1266_minimum_time_visiting_all_points.rs | Sum of Chebyshev distances |
| 1401_circle_and_rectangle_overlapping.rs | Circle/rectangle overlap via clamping |
| 1515_best_position_for_a_service_centre.rs | Geometric median with Weiszfeld iteration |
| 1610_maximum_number_of_visible_points.rs | Angular sliding window over polar angles |
| 1828_queries_on_number_of_points_inside_a_circle.rs | Points-in-circle counts per query |

## Classic algorithms
| File | Description |
|---|---|
| closest_pair_of_points.rs | Divide-and-conquer closest pair, O(n log n) |
| graham_scan_convex_hull.rs | Graham scan convex hull (CCW, no collinear) |
| line_line_intersection.rs | Intersection point of lines and segments (parametric) |
| minimum_enclosing_circle_welzl.rs | Smallest enclosing circle, randomized incremental |
| picks_theorem_lattice_points.rs | Interior/boundary lattice points via Pick's theorem |
| point_in_polygon.rs | Inside/outside/boundary classification by ray casting |
| polygon_area_shoelace.rs | Polygon area and orientation via shoelace formula |
| rotating_calipers_diameter.rs | Farthest pair via convex hull and rotating calipers |
| segment_intersection.rs | Exact integer segment intersection test |
| sutherland_hodgman_polygon_clipping.rs | Clip a polygon against a convex polygon |

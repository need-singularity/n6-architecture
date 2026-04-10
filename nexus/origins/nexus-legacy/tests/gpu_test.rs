use nexus::gpu::fallback;

#[test]
fn test_distance_matrix_cpu() {
    // 3 points in 2D: [0,0], [3,4], [1,0]
    let data = vec![0.0, 0.0, 3.0, 4.0, 1.0, 0.0];
    let dist = fallback::distance_matrix_cpu(&data, 3, 2);

    // Lower triangle: (1,0), (2,0), (2,1)
    // dist(1,0) = sqrt(9+16) = 5.0
    // dist(2,0) = sqrt(1+0)  = 1.0
    // dist(2,1) = sqrt(4+16) = sqrt(20) ≈ 4.4721
    assert_eq!(dist.len(), 3);
    assert!((dist[0] - 5.0).abs() < 1e-5, "dist(1,0) = {}", dist[0]);
    assert!((dist[1] - 1.0).abs() < 1e-5, "dist(2,0) = {}", dist[1]);
    assert!(
        (dist[2] - 20.0f32.sqrt()).abs() < 1e-4,
        "dist(2,1) = {}",
        dist[2]
    );
}

#[test]
fn test_knn_cpu() {
    // 3 points: [0,0], [3,4], [1,0]
    // Distances: d(1,0)=5, d(2,0)=1, d(2,1)=sqrt(20)≈4.47
    let data = vec![0.0, 0.0, 3.0, 4.0, 1.0, 0.0];
    let dist = fallback::distance_matrix_cpu(&data, 3, 2);
    let knn = fallback::knn_cpu(&dist, 3, 2);

    // k=2 means 2 nearest neighbors per point (all others since n=3)
    assert_eq!(knn.len(), 6); // 3 points * 2 neighbors

    // Point 0: nearest = point 2 (d=1), then point 1 (d=5)
    assert_eq!(knn[0], 2);
    assert_eq!(knn[1], 1);

    // Point 1: nearest = point 2 (d≈4.47), then point 0 (d=5)
    assert_eq!(knn[2], 2);
    assert_eq!(knn[3], 0);

    // Point 2: nearest = point 0 (d=1), then point 1 (d≈4.47)
    assert_eq!(knn[4], 0);
    assert_eq!(knn[5], 1);
}

#[test]
fn test_mutual_info_cpu() {
    let n = 200u32;
    let d = 2u32;

    // Correlated data: dim1 ≈ dim0
    let mut correlated = Vec::with_capacity((n * d) as usize);
    for i in 0..n {
        let x = i as f32 / n as f32;
        correlated.push(x);
        correlated.push(x + 0.01 * ((i as f32) * 0.1).sin()); // nearly identical
    }

    let mi_corr = fallback::mutual_info_cpu(&correlated, n, d, 10);
    assert_eq!(mi_corr.len(), 4); // 2x2 matrix
    // MI(0,1) and MI(1,0) should be positive (correlated)
    assert!(
        mi_corr[0 * 2 + 1] > 0.1,
        "MI(0,1) = {} should be > 0.1 for correlated data",
        mi_corr[0 * 2 + 1]
    );
    assert!(
        mi_corr[1 * 2 + 0] > 0.1,
        "MI(1,0) = {} should be > 0.1 for correlated data",
        mi_corr[1 * 2 + 0]
    );

    // Independent data: dim0 = linear, dim1 = unrelated pattern
    let mut independent = Vec::with_capacity((n * d) as usize);
    for i in 0..n {
        let x = i as f32 / n as f32;
        // Use a pattern that creates uniform-ish distribution uncorrelated with x
        let y = ((i as f32 * 7.37 + 3.14) % 1.0).abs();
        independent.push(x);
        independent.push(y);
    }

    let mi_ind = fallback::mutual_info_cpu(&independent, n, d, 10);
    // MI should be small for independent dimensions
    assert!(
        mi_ind[0 * 2 + 1] < 0.15,
        "MI(0,1) = {} should be < 0.15 for independent data",
        mi_ind[0 * 2 + 1]
    );
}

use plotly::{Plot, Scatter};
use rand::Rng;

fn blackman_smoothing(data: &Vec<f64>, window_size: usize) -> Vec<f64> {
    let mut smoothed_data = vec![0.0; data.len()];

    // Create the Blackman window
    let mut window = vec![0.0; window_size];
    let a0 = 0.42659;
    let a1 = 0.49656;
    let a2 = 0.076849;
    for i in 0..window_size {
        let pi = std::f64::consts::PI;
        let frac = 2.0 * pi * i as f64 / (window_size - 1) as f64;
        window[i] = a0 - a1 * (frac).cos() + a2 * (2.0 * frac).cos();
    }

    // Smooth the data using the window
    let half_window_size = window_size / 2;
    let data_len = data.len();
    for i in 0..data_len {
        let mut sum = 0.0;
        let start_idx = if i >= half_window_size { i - half_window_size } else { 0 };
        let end_idx = if i + half_window_size < data_len { i + half_window_size } else { data_len - 1 };
        let mut weight_sum = 0.0;
        for j in start_idx..=end_idx {
            let idx_diff = j as isize - i as isize;
            let weight = if idx_diff >= -(half_window_size as isize) && idx_diff <= half_window_size as isize {
                window[(idx_diff + half_window_size as isize) as usize]
            } else {
                0.0
            };
            sum += data[j] * weight;
            weight_sum += weight;
        }
        smoothed_data[i] = sum / weight_sum;
    }

    smoothed_data
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut data:Vec<f64> = vec![0.0];
    let mut x: Vec<f64> = vec![0.0];
    for i in 0..10000{
        data.push(rng.gen_range(0.0..50.0));
        x.push(i as f64);

    }

    let smoothed_data = blackman_smoothing(&data, 11);
    println!("Smoothed data: {:?}", smoothed_data);


    let mut plot = Plot::new();
    let trace = Scatter::new(x.clone(), smoothed_data);
    let trace_1 = Scatter::new(x, data);
    plot.add_trace(trace);
    plot.add_trace(trace_1);

    plot.write_html("out.html");
}




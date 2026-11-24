use crate::metrics::BenchmarkMetrics;
use crate::scenario::BenchmarkScenario;

/// Generator for single-page responsive HTML benchmark reports.
pub struct BenchmarkHtmlReporter;

impl BenchmarkHtmlReporter {
    /// Render benchmark results and scenario breakdown as standalone HTML.
    pub fn render(scenario: &BenchmarkScenario, step_metrics: &[BenchmarkMetrics]) -> String {
        let mut rows = String::new();
        let mut labels = String::new();
        let mut cpu_data = String::new();
        let mut mem_data = String::new();

        for (i, step) in scenario.steps.iter().enumerate() {
            let metrics = step_metrics.get(i).cloned().unwrap_or_default();
            rows.push_str(&format!(
                "<tr>\n\
                    <td><strong>{}</strong></td>\n\
                    <td><code>{}</code></td>\n\
                    <td>{}</td>\n\
                    <td>{}</td>\n\
                    <td>{} B</td>\n\
                    <td>{} stroops</td>\n\
                </tr>\n",
                step.name,
                step.contract_method,
                step.iterations,
                metrics.cpu_instructions,
                metrics.memory_bytes,
                metrics.estimated_fee_stroops
            ));

            if i > 0 {
                labels.push_str(", ");
                cpu_data.push_str(", ");
                mem_data.push_str(", ");
            }
            labels.push_str(&format!("'{}'", step.name));
            cpu_data.push_str(&metrics.cpu_instructions.to_string());
            mem_data.push_str(&metrics.memory_bytes.to_string());
        }

        format!(
            "<!DOCTYPE html>\n\
            <html lang=\"en\">\n\
            <head>\n\
            <meta charset=\"UTF-8\">\n\
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
            <title>Soroban Bench Report - {name}</title>\n\
            <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n\
            <style>\n\
            body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 2rem; background: #0f172a; color: #e2e8f0; }}\n\
            .container {{ max-width: 1000px; margin: 0 auto; }}\n\
            header {{ border-bottom: 1px solid #334155; padding-bottom: 1.5rem; margin-bottom: 2rem; }}\n\
            h1 {{ margin: 0 0 0.5rem 0; color: #38bdf8; }}\n\
            .chart-card {{ background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 1.5rem; margin-bottom: 2rem; }}\n\
            table {{ width: 100%; border-collapse: collapse; background: #1e293b; border-radius: 8px; overflow: hidden; border: 1px solid #334155; }}\n\
            th, td {{ padding: 0.75rem 1rem; text-align: left; border-bottom: 1px solid #334155; }}\n\
            th {{ background: #0f172a; color: #94a3b8; font-size: 0.85rem; text-transform: uppercase; }}\n\
            code {{ font-family: 'Fira Code', monospace; color: #a5f3fc; }}\n\
            </style>\n\
            </head>\n\
            <body>\n\
            <div class=\"container\">\n\
            <header>\n\
                <h1>Soroban Bench Performance Report</h1>\n\
                <p>Scenario: <strong>{name}</strong> — {desc}</p>\n\
            </header>\n\
            <div class=\"chart-card\">\n\
                <canvas id=\"perfChart\"></canvas>\n\
            </div>\n\
            <table>\n\
                <thead>\n\
                    <tr>\n\
                        <th>Step</th><th>Method</th><th>Iterations</th><th>CPU Instructions</th><th>Memory</th><th>Est. Fee</th>\n\
                    </tr>\n\
                </thead>\n\
                <tbody>\n\
                    {rows}\n\
                </tbody>\n\
            </table>\n\
            </div>\n\
            <script>\n\
            const ctx = document.getElementById('perfChart').getContext('2d');\n\
            new Chart(ctx, {{\n\
                type: 'bar',\n\
                data: {{\n\
                    labels: [{labels}],\n\
                    datasets: [\n\
                        {{ label: 'CPU Instructions', data: [{cpu_data}], backgroundColor: '#38bdf8' }},\n\
                        {{ label: 'Memory (Bytes)', data: [{mem_data}], backgroundColor: '#818cf8' }}\n\
                    ]\n\
                }},\n\
                options: {{\n\
                    responsive: true,\n\
                    plugins: {{ legend: {{ labels: {{ color: '#e2e8f0' }} }} }},\n\
                    scales: {{ x: {{ ticks: {{ color: '#94a3b8' }} }}, y: {{ ticks: {{ color: '#94a3b8' }} }} }}\n\
                }}\n\
            }});\n\
            </script>\n\
            </body>\n\
            </html>",
            name = scenario.name,
            desc = scenario.description,
            rows = rows,
            labels = labels,
            cpu_data = cpu_data,
            mem_data = mem_data
        )
    }
}

<script lang="ts">
    import { onMount } from 'svelte';
    import { Chart, registerables } from 'chart.js';

    Chart.register(...registerables);

    let { labels, data, unit = "hours" }: {
        labels: string[];
        data: number[];
        unit?: string;
    } = $props();

    let canvas: HTMLCanvasElement;
    let chart: Chart;

    function getChartColor(varName: string): string {
        return getComputedStyle(document.documentElement).getPropertyValue(varName).trim();
    }

    function buildChart() {
        if (chart) chart.destroy();
        const primary = getChartColor('--chart-primary');
        const neutral = getChartColor('--chart-neutral');
        const text = getChartColor('--chart-text');

        chart = new Chart(canvas, {
            type: 'bar',
            data: {
                labels,
                datasets: [{
                    data,
                    backgroundColor: primary + '99',
                    borderColor: primary,
                    borderWidth: 1,
                    borderRadius: 8,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        callbacks: {
                            label: (ctx) => `${(ctx.parsed.y ?? 0).toFixed(1)} ${unit}`
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        grid: { color: neutral + '40' },
                        ticks: { color: text, font: { family: 'Inter', size: 11 } }
                    },
                    x: {
                        grid: { display: false },
                        ticks: { color: text, font: { family: 'Inter', size: 11 } }
                    }
                }
            }
        });
    }

    onMount(() => {
        buildChart();
        const observer = new MutationObserver(() => buildChart());
        observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
        return () => { chart.destroy(); observer.disconnect(); };
    });
</script>

<div class="relative w-full h-full min-h-[200px]">
    <canvas bind:this={canvas}></canvas>
</div>

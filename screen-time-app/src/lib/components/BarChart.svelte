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

    onMount(() => {
        const chart = new Chart(canvas, {
            type: 'bar',
            data: {
                labels,
                datasets: [{
                    data,
                    backgroundColor: 'rgba(0, 88, 188, 0.6)',
                    borderColor: 'rgba(0, 88, 188, 1)',
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
                        grid: { color: 'rgba(0,0,0,0.05)' },
                        ticks: { font: { family: 'Inter', size: 11 } }
                    },
                    x: {
                        grid: { display: false },
                        ticks: { font: { family: 'Inter', size: 11 } }
                    }
                }
            }
        });

        return () => chart.destroy();
    });
</script>

<div class="relative w-full h-full min-h-[200px]">
    <canvas bind:this={canvas}></canvas>
</div>

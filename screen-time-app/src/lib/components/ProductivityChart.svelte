<script lang="ts">
    import { onMount } from 'svelte';
    import { Chart, registerables } from 'chart.js';

    Chart.register(...registerables);

    let { data }: {
        data: Array<{ day: string; productive: number; neutral: number; leisure: number }>;
    } = $props();

    let canvas: HTMLCanvasElement;

    onMount(() => {
        const chart = new Chart(canvas, {
            type: 'bar',
            data: {
                labels: data.map(d => d.day),
                datasets: [
                    {
                        label: 'Productive',
                        data: data.map(d => d.productive / 3600),
                        backgroundColor: 'rgba(0, 88, 188, 0.8)',
                        borderRadius: { topLeft: 0, topRight: 0, bottomLeft: 4, bottomRight: 4 },
                    },
                    {
                        label: 'Neutral',
                        data: data.map(d => d.neutral / 3600),
                        backgroundColor: 'rgba(227, 226, 231, 0.8)',
                    },
                    {
                        label: 'Leisure',
                        data: data.map(d => d.leisure / 3600),
                        backgroundColor: 'rgba(76, 74, 202, 0.8)',
                        borderRadius: { topLeft: 4, topRight: 4, bottomLeft: 0, bottomRight: 0 },
                    },
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        position: 'top',
                        labels: { font: { family: 'Inter', size: 11 }, usePointStyle: true, pointStyle: 'circle' }
                    },
                },
                scales: {
                    x: { stacked: true, grid: { display: false }, ticks: { font: { family: 'Inter', size: 11 } } },
                    y: { stacked: true, beginAtZero: true, grid: { color: 'rgba(0,0,0,0.05)' }, ticks: { font: { family: 'Inter', size: 11 } } },
                }
            }
        });

        return () => chart.destroy();
    });
</script>

<h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Work vs. Leisure</h3>
<div class="relative h-48 mt-md">
    <canvas bind:this={canvas}></canvas>
</div>

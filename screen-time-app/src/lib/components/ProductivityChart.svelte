<script lang="ts">
    import { onMount } from 'svelte';
    import { Chart, registerables } from 'chart.js';

    Chart.register(...registerables);

    let { data }: {
        data: Array<{ day: string; productive: number; neutral: number; leisure: number }>;
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
        const tertiary = getChartColor('--chart-tertiary');
        const text = getChartColor('--chart-text');

        chart = new Chart(canvas, {
            type: 'bar',
            data: {
                labels: data.map(d => d.day),
                datasets: [
                    {
                        label: 'Productive',
                        data: data.map(d => d.productive / 3600),
                        backgroundColor: primary + 'cc',
                        borderRadius: { topLeft: 0, topRight: 0, bottomLeft: 4, bottomRight: 4 },
                    },
                    {
                        label: 'Neutral',
                        data: data.map(d => d.neutral / 3600),
                        backgroundColor: neutral + 'cc',
                    },
                    {
                        label: 'Leisure',
                        data: data.map(d => d.leisure / 3600),
                        backgroundColor: tertiary + 'cc',
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
                        labels: { color: text, font: { family: 'Inter', size: 11 }, usePointStyle: true, pointStyle: 'circle' }
                    },
                },
                scales: {
                    x: { stacked: true, grid: { display: false }, ticks: { color: text, font: { family: 'Inter', size: 11 } } },
                    y: { stacked: true, beginAtZero: true, grid: { color: neutral + '40' }, ticks: { color: text, font: { family: 'Inter', size: 11 } } },
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

<h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Work vs. Leisure</h3>
<div class="relative h-48 mt-md">
    <canvas bind:this={canvas}></canvas>
</div>

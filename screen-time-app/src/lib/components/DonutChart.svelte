<script lang="ts">
    import { onMount } from 'svelte';
    import { Chart, registerables } from 'chart.js';

    Chart.register(...registerables);

    let { labels, data, colors }: {
        labels: string[];
        data: number[];
        colors: string[];
    } = $props();

    let canvas: HTMLCanvasElement;
    let chart: Chart;

    function resolveColor(color: string): string {
        if (color.startsWith('--')) {
            return getComputedStyle(document.documentElement).getPropertyValue(color).trim();
        }
        return color;
    }

    function buildChart() {
        if (chart) chart.destroy();
        const resolvedColors = colors.map(resolveColor);

        chart = new Chart(canvas, {
            type: 'doughnut',
            data: {
                labels,
                datasets: [{
                    data,
                    backgroundColor: resolvedColors,
                    borderWidth: 0,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: true,
                cutout: '65%',
                plugins: {
                    legend: { display: false },
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

<div class="relative w-32 h-32 flex-shrink-0">
    <canvas bind:this={canvas}></canvas>
</div>

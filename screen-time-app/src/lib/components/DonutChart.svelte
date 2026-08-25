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

    onMount(() => {
        const chart = new Chart(canvas, {
            type: 'doughnut',
            data: {
                labels,
                datasets: [{
                    data,
                    backgroundColor: colors,
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

        return () => chart.destroy();
    });
</script>

<div class="relative w-32 h-32 flex-shrink-0">
    <canvas bind:this={canvas}></canvas>
</div>

<script lang="ts" context="module">
    import * as Highcharts from 'highcharts'
    import HighchartsMore from 'highcharts/highcharts-more'
    import HighchartsSolidGauge from 'highcharts/modules/solid-gauge'
    // Initialize modules
    HighchartsMore(Highcharts)
    HighchartsSolidGauge(Highcharts)

</script>
<script lang="ts">
    import {onMount} from "svelte";

    // The data used.
    export let used: number;
    // The data allocated.
    export let allocated: number;
    // The data total.
    export let total: number;
    // The div in which we'll render the chart
    let chartDiv: HTMLDivElement|undefined = undefined

    onMount(() => {
        if (undefined === chartDiv) return;
        // Create the "Used" series
        const usedSeries: Highcharts.SeriesGaugeOptions = {
            type: "gauge",
            name: "Used",
            data: [50]
        }
        // Create the "Allocated" series
        const allocatedSeries: Highcharts.SeriesGaugeOptions = {
            type: "gauge",
            name: "Allocated",
            data: [75]
        }
        // Figure out the chart options
        /*
        const chartOptions: Highcharts.Options = {
            chart: {
                type: 'solidgauge',
                height: 350
            },
            credits: { enabled: false },
            exporting: { enabled: false },
            title: null,
            tooltip: { enabled: false },
            pane: {
                center: ['50%', '80%'],
                size: '140%',
                startAngle: -90,
                endAngle: 90,
                background: [
                    {
                        backgroundColor: '#eee',
                        innerRadius: '60%',
                        outerRadius: '80%',
                        shape: 'arc'
                    }
                ]
            },
            plotOptions: {
                solidgauge: {
                    dataLabels: {
                        y: 5,
                        borderWidth: 0,
                        useHTML: true
                    }
                }
            },
            series: [
                usedSeries//, allocatedSeries
            ],
            yAxis: {
                min: 0,
                max: 100,
                stops: [
                    [0.1, '#55BF3B'], // green
                    [0.5, '#DDDF0D'], // yellow
                    [0.9, '#DF5353'] // red
                ],
                lineWidth: 10,
                tickWidth: 10,
                minorTickInterval: null,
                tickAmount: 2,
                title: {
                    y: -70,
                    text: 'Size'
                },
                labels: {
                    y: 16
                }
            },
        }*/
        const chartOptions: Highcharts.Options = {
            chart: {
                type: 'solidgauge',
                height: 340
            },
            pane: {
                center: ['50%', '85%'],
                    size: '130%',
                    startAngle: -90,
                    endAngle: 90,
                    background: {
                    backgroundColor: '#EEE',
                    innerRadius: '60%',
                    outerRadius: '100%',
                    shape: 'arc'
                }
            },
            // Don't show the "highcharts.com" branding
            credits: { enabled: false },
            // Prevent export
            exporting: { enabled: false },
            // Don't show a title
            title: null,
            // Disable the tooltip
            tooltip: { enabled: false },
            // the value axis
            yAxis: {
                min: 0,
                max: 200,
                title: {
                    y: 10,

                    text: 'Database Size'
                },
                // Show a color gradient from good to bad
                stops: [
                    [0.1, '#55BF3B'], // green
                    [0.8, '#DDDF0D'], // yellow
                    [0.9, '#DF5353'] // red
                ],
                lineWidth: 0,
                tickWidth: 0,
                minorTickInterval: null,
                tickAmount: 2,
                labels: {
                    y: 16,
                }
            },
            plotOptions: {
                solidgauge: {
                    dataLabels: {
                        y: 5,
                        borderWidth: 0,
                        useHTML: true
                    }
                }
            },

            series: [
                {
                    name: 'Used',
                    data: [80],
                    dataLabels: null,
                    tooltip: {
                        valueSuffix: ' GB'
                    }
                },
                {
                    name: 'Allocated',
                    data: [90],
                    dataLabels: {
                        format:
                            '<div style="text-align:center">' +
                            '<span style="font-size:25px">{y}</span><br/>' +
                            '<span style="font-size:12px;opacity:0.4">km/h</span>' +
                            '</div>'
                    },
                    tooltip: {
                        valueSuffix: ' GB'
                    }
                },
            ]
        }
        // Create the chart control in the div
        const chartControl = new Highcharts.Chart(chartDiv, chartOptions, null)
    })
</script>
<div class="bg-danger">
    <div bind:this={chartDiv} class="chart-container"></div>

</div>

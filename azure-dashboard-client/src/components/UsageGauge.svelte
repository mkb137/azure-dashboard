<script lang="ts" context="module">
    import * as Highcharts from 'highcharts'
    import HighchartsMore from 'highcharts/highcharts-more'
    import HighchartsSolidGauge from 'highcharts/modules/solid-gauge'
    // Initialize modules
    HighchartsMore(Highcharts)
    HighchartsSolidGauge(Highcharts)

</script>
<script lang="ts">
    import * as lodash from 'lodash'
    import {onMount} from "svelte";

    // The data used.
    export let used: number;
    // The data allocated.
    export let allocated: number;
    // The data total.
    export let total: number;

    // Compute the used percent of the total
    const usedFraction = lodash.round( used / total, 2 )
    // Compute the allocated percent of the total
    const allocatedFraction = lodash.round( allocated / total, 2 )
    console.log(` - used fraction = ${usedFraction}, allocatedFraction = ${allocatedFraction}`)

    // The width of the gauge
    const SVG_WIDTH = 100;
    // The height of the gauge
    const SVG_HEIGHT = 50;
    // THe width of the gauge arc
    const CURVE_WIDTH = 20;

    /**
     * Creates an SVG path from the points and markers given
     * @param values - An array of points ("[0,1]"), values ("2"), or markers ("'m'").
     * @returns A string containing the SVG points, e.g. "M 0,1 L 1,2 Z"
     */
    const toSvgPath = (values: Array<any>): string => {
        let path = ''
        for (let i = 0; i < values.length; i++) {
            const value = values[i]
            if (0 < i) {
                path += ' '
            }
            // If the value is an array...
            if (value instanceof Array) {
                // Treat it like an x,y value
                path += value[0]
                path += ' '
                path += value[1]
            } else {
                // Otherwise (e.g. if it's a marker like "M") add it as-is
                path += value
            }
        }
        return path
    }

    // Computes the location of a point on an arc relative to the origin given diameter and angle.
    const computePoint = (originX: number, originY: number, diameter: number, angle: number) => {
        // Get the change in X, which is the cos of the angle times the diameter. And because we're going to the left, we have to flip it.
        const dx = lodash.round( Math.cos(angle) * diameter * -1.0, 5 )
        // Get the change in Y, which is the sin of the angle times the diameter
        const dy = lodash.round( Math.sin(angle) * diameter * -1.0, 5 )
        // Get a point that is off the origin by the change
        const x = originX + dx
        const y = originY + dy
        console.log(` - diameter = ${diameter}, angle = ${angle}, dx = ${dx}, dy = ${dy}`)
        // Return the point
        return [x,y]
    }

    const computePath = (fromFraction : number, toFraction: number) => {
        console.log(`computePath - from = ${fromFraction}, to = ${toFraction}`)
        // The outer diameter is the SVG height
        const outerDiameter = SVG_WIDTH / 2.0 - 2
        // The inner diameter is the outer diameter minus the curve width
        const innerDiameter = outerDiameter - CURVE_WIDTH
        // Get the "from" and "to" fractions as fractions of 180 degrees, in radians
        const oneHundredAndEightyDegrees = Math.PI
        const fromAngle = fromFraction * oneHundredAndEightyDegrees
        const toAngle = toFraction * oneHundredAndEightyDegrees
        console.log(` - outerDiameter = ${outerDiameter}, innerDiameter = ${innerDiameter}, fromAngle = ${fromAngle}, toAngle = ${toAngle}`)
        // Get our arc origin, which is the bottom middle.
        const originX = SVG_WIDTH / 2.0;
        // noinspection UnnecessaryLocalVariableJS
        const originY = SVG_HEIGHT - 1;
        // Get the points around the curve
        const fromOuter = computePoint(originX, originY, outerDiameter, fromAngle)
        const toOuter = computePoint(originX, originY, outerDiameter, toAngle)
        const fromInner = computePoint(originX, originY, innerDiameter, fromAngle)
        const toInner = computePoint(originX, originY, innerDiameter, toAngle)
        console.log(` - fromOuter = ${fromOuter}, toOuter = ${toOuter}, fromInner = ${fromInner}, toInner = ${toInner}`)
        // Create the path elements
        const pathElements: Array<any> = [
            'M',
            fromOuter,
            // Draw the first, outer curve
            'A',
            [outerDiameter, outerDiameter],
            0, // offset
            0, // large arc
            1, // sweep direction
            toOuter, // destination
            // Draw the line closing the end of the outer curve
            'L',
            toInner,
            // Draw the second, inner curve backwards
            'A',
            [innerDiameter, innerDiameter],
            0, // offset
            0, // large arc
            0, // sweep direction
            fromInner, // destination
            // Draw the line closing the start of the arc
            'L',
            fromOuter
        ]
        // Convert the elements to an SVG path
        const path = toSvgPath(pathElements)
        console.log(` - path = ${path}`)
        return path
    }
</script>
<div class="">
    <svg viewBox="0 0 100 50">
        <g transform="">
            <!-- Draw the fill -->
            <path d={computePath(0.0,1.0)} stroke="none" fill="#eee"/>
            <!-- Draw the usage bar from 0 to the used fraction -->
            <path d={computePath(0.0,usedFraction)} stroke="none" fill="pink"/>
            <!-- Draw the allocation bar from the used fraction to the allocation fraction -->
            <path d={computePath(usedFraction,allocatedFraction)} stroke="none" fill="red"/>
            <!-- Draw the outer border -->
            <path d={computePath(0.0,1.0)} stroke="#ddd" fill="none" stroke-width="0.5"/>
        </g>
    </svg>

</div>

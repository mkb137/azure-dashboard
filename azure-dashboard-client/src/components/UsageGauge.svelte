<script lang="ts">
    import * as lodash from 'lodash'

    // The width of the gauge
    const SVG_WIDTH = 380;
    // The height of the gauge
    const SVG_HEIGHT = SVG_WIDTH / 2.0;
    // THe width of the gauge arc
    const CURVE_WIDTH = SVG_WIDTH / 7.0;

    // Converts a fraction to an HSL color between 0 = green and 1 = red
    const toColor = (fraction: number): string => {
        // Set the min and max hues
        let hueMin = 110 // green
        let hueMax = 0 // red
        // We want to scale the value exponentially towards the end
        let delta = (hueMax - hueMin) * Math.pow(fraction, 2)
        // Get the final hue
        let hue = lodash.round( hueMin + delta, 0 )
        // Create an HSL color from the result
        return `hsl(${hue}, 90%, 50%)`
    }

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

    // The data used.
    export let used: number = 0;
    // The data allocated.
    export let allocated: number = 0;
    // The data total.
    export let total: number = 0;

    // Get the values in GB
    const usedGb = used / Math.pow(2,30)
    const allocatedGb = allocated / Math.pow(2,30)
    const totalGb = total / Math.pow(2,30)

    // Compute the used percent of the total
    const usedFraction = lodash.round( used / total, 2 )
    // Compute the allocated percent of the total
    const allocatedFraction = lodash.round( allocated / total, 2 )
    // Compute the colors for each
    const usedColor = toColor(usedFraction)
    const allocatedColor = toColor(allocatedFraction)
    console.log(` - allocated fraction = ${allocatedFraction}, allocated color = ${allocatedColor}`)

    // Create a unique ID for our pattern
    const patternId = lodash.uniqueId('pattern')

</script>
<div class="text-muted" style:width={`${SVG_WIDTH}px`} style:height={`${SVG_HEIGHT}px`}>
    <svg viewBox={`0 0 ${SVG_WIDTH} ${SVG_HEIGHT}`}>
        <g transform="">
            <pattern id={patternId} x="0" y="0" width="2" height="2" patternUnits="userSpaceOnUse">
                <!-- Two instances of the same checker, only positioned apart on the `x` and `y` axis -->
                <!-- We will define the `fill` in the CSS for flexible use -->
                <rect class="checker" x="0" width="1" height="1" y="0" fill={allocatedColor}></rect>
                <rect class="checker" x="1" width="1" height="1" y="1" fill={allocatedColor}></rect>
            </pattern>
            <!-- Draw the fill -->
            <path d={computePath(0.0,1.0)} stroke="none" fill="#eee"/>
            <!-- Draw the usage bar from 0 to the used fraction -->
            <path d={computePath(0.0,usedFraction)} stroke="none" fill={usedColor}/>
            <!-- Draw the allocation bar from the used fraction to the allocation fraction -->
            <path d={computePath(usedFraction,allocatedFraction)} stroke="#ddd" fill={`url(#${patternId})`}/>
            <!-- Draw the outer border -->
            <path d={computePath(0.0,1.0)} stroke="#ddd" fill="none" stroke-width="0.5"/>

            <!-- Draw the usages -->
            <text x={SVG_WIDTH / 2} y="100" class="size-value">{usedGb.toFixed(2)}</text>
            <text x={SVG_WIDTH / 2} y="100" class="size-label">&nbsp;GB used</text>
            <text x={SVG_WIDTH / 2} y="120" class="size-value">{allocatedGb.toFixed(2)}</text>
            <text x={SVG_WIDTH / 2} y="120" class="size-label">&nbsp;GB allocated</text>
            <text x={SVG_WIDTH / 2} y="140" class="size-value">{totalGb.toFixed(2)}</text>
            <text x={SVG_WIDTH / 2} y="140" class="size-label">&nbsp;GB total</text>
        </g>
    </svg>
</div>

<style lang="scss">
    text.size-value {
        font-size: 14px;
        text-anchor: end;
    }
    text.size-label {
        font-size: 14px;
        text-anchor: start;
        fill: #6c757d;
    }
</style>

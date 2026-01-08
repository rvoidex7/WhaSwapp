import math

def generate_svg():
    width = 500
    height = 500
    cx = width / 2
    cy = height / 2

    # Colors
    main_color = "#249C92"

    svg_parts = []
    svg_parts.append(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">')

    # Defs
    svg_parts.append('<defs>')

    # Gradient for Inner Ball
    svg_parts.append('<linearGradient id="ballGradient" x1="0%" y1="0%" x2="0%" y2="100%">')
    svg_parts.append(f'<stop offset="0%" stop-color="{main_color}" />')
    svg_parts.append('<stop offset="100%" stop-color="#ffffff" />')
    svg_parts.append('</linearGradient>')

    # Mask 1: Outer Ring Erasure (Inner Hole + Triangle Pit)
    svg_parts.append('<mask id="maskOuter">')
    svg_parts.append(f'<rect x="0" y="0" width="{width}" height="{height}" fill="white" />')
    # Inner Hole
    svg_parts.append(f'<circle cx="{cx}" cy="{cy}" r="147.5" fill="black" />')
    # Triangle Pit
    # Coords relative to center: (-35, 135), (0, 188), (35, 135)
    # Absolute coords:
    p1 = (cx - 35, cy + 135)
    p2 = (cx, cy + 188)
    p3 = (cx + 35, cy + 135)
    svg_parts.append(f'<path d="M{p1[0]} {p1[1]} L{p2[0]} {p2[1]} L{p3[0]} {p3[1]} Z" fill="black" />')
    svg_parts.append('</mask>')

    # Mask 2: Reuleaux Erasure (Center Hole)
    svg_parts.append('<mask id="maskInner">')
    svg_parts.append(f'<rect x="0" y="0" width="{width}" height="{height}" fill="white" />')
    svg_parts.append(f'<circle cx="{cx}" cy="{cy}" r="35" fill="black" />')
    svg_parts.append('</mask>')

    svg_parts.append('</defs>')

    # Group 1: Outer Ring + Spikes
    svg_parts.append(f'<g mask="url(#maskOuter)" fill="{main_color}">')

    # Ring
    svg_parts.append(f'<circle cx="{cx}" cy="{cy}" r="172.5" />')

    # Spikes
    ring_radius = 172.5
    r_base = ring_radius - 2
    spike_angles = [30, 50, 70, 90, 110, 130, 150]

    for deg in spike_angles:
        is_middle = (deg == 90)
        h = 75 if is_middle else 45
        base_w = 0.40 if is_middle else 0.22

        angle = math.radians(deg)

        # Calculate points
        x1 = cx + r_base * math.cos(angle - base_w/2)
        y1 = cy + r_base * math.sin(angle - base_w/2)

        x2 = cx + r_base * math.cos(angle + base_w/2)
        y2 = cy + r_base * math.sin(angle + base_w/2)

        tip_x = cx + (r_base + h) * math.cos(angle)
        tip_y = cy + (r_base + h) * math.sin(angle)

        svg_parts.append(f'<path d="M{x1:.2f} {y1:.2f} L{tip_x:.2f} {tip_y:.2f} L{x2:.2f} {y2:.2f} Z" />')

    svg_parts.append('</g>')

    # Group 2: Reuleaux Triangle
    # Transform: translate(250, 250) rotate(180) translate(-100, -115)
    # Note: SVG transform order is significant. The canvas logic was:
    # translate(centerX, centerY); rotate(PI); translate(-100, -115);
    # In SVG transform attribute, we can chain them directly.
    svg_parts.append(f'<g mask="url(#maskInner)" fill="{main_color}" transform="translate({cx}, {cy}) rotate(180) translate(-100, -115)">')
    svg_parts.append('<path d="M2 172a196 196 0 0 0 196 0A196 196 0 0 0 100 2 196 196 0 0 0 2 172z" />')
    svg_parts.append('</g>')

    # Group 3: Inner Gradient Ball
    # Center Y calculation
    main_hole_r = 35
    inner_ball_r = main_hole_r * (2/3)
    inner_ball_cy_offset = main_hole_r - inner_ball_r # = 35 - 23.333 = 11.666

    ball_cx = cx
    ball_cy = cy + inner_ball_cy_offset

    svg_parts.append(f'<circle cx="{ball_cx}" cy="{ball_cy:.3f}" r="{inner_ball_r:.3f}" fill="url(#ballGradient)" />')

    svg_parts.append('</svg>')

    return "\n".join(svg_parts)

if __name__ == "__main__":
    svg_content = generate_svg()

    # Write to assets
    with open("assets/logo.svg", "w") as f:
        f.write(svg_content)

    # Write to app icons
    with open("apps/desktop/src-tauri/icons/logo.svg", "w") as f:
        f.write(svg_content)

    print("SVG generated successfully.")

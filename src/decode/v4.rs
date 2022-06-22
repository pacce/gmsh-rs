#![allow(unused)]

use {
    crate::{
        element::{self, Elementary, Physical, Topology},
        format::Format,
        mesh::{self, Mesh},
        node::{self, Coordinate, Node},
    },
    nom::{
        bytes::complete::{tag, take_until},
        character::complete::{i32, newline, space0, space1, u64},
        combinator::{all_consuming, complete, cond, map, map_parser, opt, peek},
        error::ParseError,
        multi::{count, length_count},
        number::complete::double,
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    },
    std::collections::HashMap,
};

pub fn mesh<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Mesh, E> {
    let (i, f) = terminated(
        map_parser(
            block("$MeshFormat\n", "\n$EndMeshFormat"),
            all_consuming(format),
        ),
        newline,
    )(i)?;

    let (i, _physical_names) = opt(terminated(physical_names, newline))(i)?;
    let (i, _entities) = opt(terminated(entities, newline))(i)?;
    let (i, _partitioned_entities) = opt(terminated(partitioned_entities, newline))(i)?;

    let (i, nodes) = terminated(nodes, newline)(i)?;

    let (i, elements) = terminated(
        map_parser(
            block("$Elements\n", "$EndElements"),
            all_consuming(elements),
        ),
        newline,
    )(i)?;

    Ok((i, Mesh::new(Some(f), nodes, elements)))
}

fn block<'a, E: ParseError<&'a str>>(
    startblock: &'static str,
    endblock: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> + 'a {
    move |i| delimited(tag(startblock), take_until(endblock), tag(endblock))(i)
}

fn space0newline<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    map(pair(space0, newline), |_| ())(i)
}

fn format<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Format, E> {
    let (i, (v, _, f, _, s)) = tuple((double, space1, i32, space1, i32))(i)?;

    let format = Format::new(v, f, s);
    Ok((i, format))
}

fn physical_name<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (i32, i32, &'a str), E> {
    let name_sep = "\"";
    map(
        tuple((
            i32,
            space1,
            i32,
            space1,
            delimited(tag(name_sep), take_until(name_sep), tag(name_sep)),
        )),
        |(dimension, _, tag, _, name)| (dimension, tag, name),
    )(i)
}

fn physical_names<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<(i32, i32, &'a str)>, E> {
    let (i, _) = terminated(tag("$PhysicalNames"), newline)(i)?;

    let (i, physical_names) =
        length_count(terminated(u64, newline), terminated(physical_name, newline))(i)?;
    let (i, _) = tag("$EndPhysicalNames")(i)?;

    Ok((i, physical_names))
}

#[derive(Debug, Clone)]
struct PointTag {
    tag: node::Id,
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
    physical_tags: Vec<node::Id>,
}

fn point_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PointTag, E> {
    let (i, (tag, _, x, _, y, _, z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        PointTag {
            tag,
            x,
            y,
            z,
            physical_tags,
        },
    ))
}

#[derive(Debug, Clone)]
struct CurveTag {
    tag: node::Id,
    min_x: Coordinate,
    min_y: Coordinate,
    min_z: Coordinate,
    max_x: Coordinate,
    max_y: Coordinate,
    max_z: Coordinate,
    physical_tags: Vec<node::Id>,
    point_tags: Vec<node::Id>,
}

fn curve_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, CurveTag, E> {
    let (i, (tag, _, min_x, _, min_y, _, min_z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, point_tags) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        CurveTag {
            tag,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            point_tags,
        },
    ))
}

#[derive(Debug, Clone)]
struct SurfaceTag {
    tag: node::Id,
    min_x: Coordinate,
    min_y: Coordinate,
    min_z: Coordinate,
    max_x: Coordinate,
    max_y: Coordinate,
    max_z: Coordinate,
    physical_tags: Vec<node::Id>,
    curve_tags: Vec<node::Id>,
}

fn surface_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, SurfaceTag, E> {
    let (i, (tag, _, min_x, _, min_y, _, min_z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, curve_tags) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        SurfaceTag {
            tag,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            curve_tags,
        },
    ))
}

#[derive(Debug, Clone)]
struct VolumeTag {
    tag: node::Id,
    min_x: Coordinate,
    min_y: Coordinate,
    min_z: Coordinate,
    max_x: Coordinate,
    max_y: Coordinate,
    max_z: Coordinate,
    physical_tags: Vec<node::Id>,
    surface_tags: Vec<node::Id>,
}

fn volume_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, VolumeTag, E> {
    let (i, (tag, _, min_x, _, min_y, _, min_z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, surface_tags) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        VolumeTag {
            tag,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            surface_tags,
        },
    ))
}

fn entities<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    let (i, _) = terminated(tag("$Entities"), newline)(i)?;

    let (i, (npoints, _, ncurves, _, nsurfaces, _, nvolumes, _)) =
        tuple((u64, space1, u64, space1, u64, space1, u64, newline))(i)?;

    let (i, _point_tags) = count(terminated(point_tag, space0newline), npoints as usize)(i)?;

    let (i, _curve_tags) = count(terminated(curve_tag, space0newline), ncurves as usize)(i)?;
    let (i, _surface_tags) = count(terminated(surface_tag, space0newline), nsurfaces as usize)(i)?;
    let (i, _volume_tags) = count(terminated(volume_tag, space0newline), nvolumes as usize)(i)?;

    let (i, _) = take_until("$EndEntities")(i)?;

    let (i, _) = tag("$EndEntities")(i)?;

    Ok((i, ()))
}

fn partitioned_entities<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    let (_i, _) = terminated(tag("$PartitionedEntities"), newline)(i)?;
    todo!()
}

fn coordinate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Coordinate, E> {
    double(i)
}

fn id<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, node::Id, E> {
    i32(i)
}

fn node<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (node::Id, Node), E> {
    let (i, id) = terminated(id, space1)(i)?;

    let (i, x) = terminated(coordinate, space1)(i)?;
    let (i, y) = terminated(coordinate, space1)(i)?;
    let (i, z) = terminated(coordinate, newline)(i)?;

    let node = Node::new(x, y, z);
    Ok((i, (id, node)))
}

struct EntityBlock {
    dim: i32,
    tag: node::Id,
    parametric: bool,
    tags: Vec<node::Id>,
    positions: Vec<(
        Coordinate,
        Coordinate,
        Coordinate,
        Option<Coordinate>,
        Option<Coordinate>,
        Option<Coordinate>,
    )>,
}

fn entityblock<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, EntityBlock, E> {
    let (i, (dim, _, tag, _, parametric, _, num_nodes, _)) =
        tuple((i32, space1, id, space1, i32, space1, u64, newline))(i)?;
    let parametric = parametric == 1;

    let (i, tags) = count(terminated(id, newline), num_nodes as usize)(i)?;
    let (i, positions) = count(
        map(
            terminated(
                tuple((
                    coordinate,
                    space1,
                    coordinate,
                    space1,
                    coordinate,
                    cond(parametric && dim >= 1, preceded(space1, coordinate)),
                    cond(parametric && dim >= 2, preceded(space1, coordinate)),
                    cond(parametric && dim == 3, preceded(space1, coordinate)),
                )),
                newline,
            ),
            |(x, _, y, _, z, u, v, w)| (x, y, z, u, v, w),
        ),
        num_nodes as usize,
    )(i)?;

    // This block must skip all newlines
    if num_nodes == 0 {
        let (i, _) = newline(i)?;
    }

    Ok((
        i,
        EntityBlock {
            dim,
            tag,
            parametric,
            tags,
            positions,
        },
    ))
}

fn nodes<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, mesh::Nodes, E> {
    let (i, _) = terminated(tag("$Nodes"), newline)(i)?;

    let (i, (num_ent_blocks, _, num_nodes, _, min_node_tag, _, max_node_tag, _)) =
        tuple((u64, space1, u64, space1, id, space1, id, newline))(i)?;

    let (i, entities) = count(entityblock, num_ent_blocks as usize)(i)?;

    let (i, _) = tag("$EndNodes")(i)?;

    let mut nodes: mesh::Nodes = HashMap::new();
    for entity in entities {
        for (id, node) in entity.tags.iter().zip(entity.positions) {
            let node = Node::new(node.0, node.1, node.2);
            nodes.insert(*id, node);
        }
    }

    Ok((i, nodes))
}

fn topology<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, i32, E> {
    i32(i)
}

fn physical<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Physical, E> {
    i32(i)
}

fn elementary<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Elementary, E> {
    i32(i)
}

fn element_parser<'a, E: ParseError<&'a str>>(
    typ: i32,
) -> impl FnMut(&'a str) -> IResult<&'a str, (Physical, Elementary, Topology), E> {
    move |i| {
        let (i, (elemental_tag, _, physical_tag)) = tuple((id, space1, peek(id)))(i)?;
        let (i, topology) = match typ {
            1 => line(i),
            2 => triangle3(i),
            3 => quadrangle4(i),
            4 => tetrahedron4(i),
            5 => hexahedron8(i),
            15 => point(i),
            x => unimplemented!("Element type {}", x),
        }?;

        Ok((i, (physical_tag, elemental_tag, topology)))
    }
}

fn element_group<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<(element::Id, Physical, Elementary, Topology)>, E> {
    let (i, (entity_dim, _, tag, _, typ, _, num_elements_in_block, _)) =
        tuple((i32, space1, id, space1, i32, space1, u64, newline))(i)?;

    count(
        map(
            terminated(element_parser(typ), space0newline),
            move |(physical, elementary, topology)| (tag, physical, elementary, topology),
        ),
        num_elements_in_block as usize,
    )(i)
}

fn elements<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, mesh::Elements, E> {
    let (i, (num_entity_blocks, _, num_elements, _, min_tag, _, max_tag, _)) =
        tuple((u64, space1, u64, space1, id, space1, id, newline))(i)?;

    let (i, elems) = count(element_group, num_entity_blocks as usize)(i)?;

    let mut elements: mesh::Elements = HashMap::new();
    for elems in elems {
        for (id, p, e, t) in elems {
            elements.insert(id, (p, e, t));
        }
    }

    Ok((i, elements))
}

// Element parser

fn line<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, (x0, x1)) = separated_pair(id, space1, id)(i)?;

    Ok((i, Topology::Line2(x0, x1)))
}

fn triangle3<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = terminated(id, space1)(i)?;
    let (i, x1) = terminated(id, space1)(i)?;
    let (i, x2) = id(i)?;

    Ok((i, Topology::Triangle3(x0, x1, x2)))
}

fn quadrangle4<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = terminated(id, space1)(i)?;
    let (i, x1) = terminated(id, space1)(i)?;
    let (i, x2) = terminated(id, space1)(i)?;
    let (i, x3) = id(i)?;

    Ok((i, Topology::Quadrangle4(x0, x1, x2, x3)))
}

fn tetrahedron4<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = terminated(id, space1)(i)?;
    let (i, x1) = terminated(id, space1)(i)?;
    let (i, x2) = terminated(id, space1)(i)?;
    let (i, x3) = id(i)?;

    Ok((i, Topology::Tetrahedron4(x0, x1, x2, x3)))
}

fn hexahedron8<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = terminated(id, space1)(i)?;
    let (i, x1) = terminated(id, space1)(i)?;
    let (i, x2) = terminated(id, space1)(i)?;
    let (i, x3) = terminated(id, space1)(i)?;
    let (i, x4) = terminated(id, space1)(i)?;
    let (i, x5) = terminated(id, space1)(i)?;
    let (i, x6) = terminated(id, space1)(i)?;
    let (i, x7) = id(i)?;

    Ok((i, Topology::Hexahedron8(x0, x1, x2, x3, x4, x5, x6, x7)))
}

fn point<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    map(id, Topology::Point1)(i)
}

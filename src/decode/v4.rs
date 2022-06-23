//! https://gmsh.info/doc/texinfo/gmsh.html#MSH-file-format

use {
    crate::{
        element::Topology,
        mesh,
        node::{self, Coordinate, Id},
    },
    nom::{
        bytes::complete::{tag, take_until},
        character::complete::{i32, newline, space0, space1, u64},
        combinator::{all_consuming, cond, map, map_parser, opt},
        error::ParseError,
        multi::{count, length_count},
        number::complete::double,
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct Format {
    pub(crate) version: String,
    pub(crate) file: i32,
    pub(crate) size: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Position {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) u: Option<f64>,
    pub(crate) v: Option<f64>,
    pub(crate) w: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeEntity {
    pub(crate) dim: i32,
    pub(crate) tag: Id,
    pub(crate) node_tags: Vec<Id>,
    pub(crate) node_positions: Vec<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Nodes {
    pub(crate) min_node: Id,
    pub(crate) max_node: Id,
    pub(crate) num_nodes: u64,
    pub(crate) entities: Vec<NodeEntity>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaggedTopology {
    pub(crate) tag: i32,
    pub(crate) topology: Topology,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElementEntity {
    pub(crate) dim: i32,
    pub(crate) tag: Id,
    pub(crate) elements: Vec<TaggedTopology>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Elements {
    pub(crate) num_elements: u64,
    pub(crate) min_tag: Id,
    pub(crate) max_tag: Id,
    pub(crate) entities: Vec<ElementEntity>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhysicalName {
    pub(crate) dimension: i32,
    pub(crate) tag: Id,
    pub(crate) name: String,
}

pub(crate) type PhysicalNames = Vec<PhysicalName>;

#[derive(Clone, Debug, PartialEq)]
pub struct EntityPoint {
    pub(crate) tag: Id,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) physical_tags: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityCurve {
    pub(crate) tag: Id,
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
    pub(crate) physical_tags: Vec<Id>,
    pub(crate) bounding_points: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntitySurface {
    pub(crate) tag: Id,
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
    pub(crate) physical_tags: Vec<Id>,
    pub(crate) bounding_curves: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityVolume {
    pub(crate) tag: Id,
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
    pub(crate) physical_tags: Vec<Id>,
    pub(crate) bounding_surfaces: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entities {
    pub(crate) points: Vec<EntityPoint>,
    pub(crate) curves: Vec<EntityCurve>,
    pub(crate) surfaces: Vec<EntitySurface>,
    pub(crate) volumes: Vec<EntityVolume>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityGhost {
    pub(crate) tag: Id,
    pub(crate) partition: Id,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionedEntityPoint {
    pub(crate) tag: Id,
    pub(crate) parent_dim: i32,
    pub(crate) parent_tag: Id,
    pub(crate) partition_tags: Vec<Id>,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) physical_tags: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionedEntityCurve {
    pub(crate) tag: Id,
    pub(crate) parent_dim: i32,
    pub(crate) parent_tag: Id,
    pub(crate) partition_tags: Vec<Id>,
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
    pub(crate) physical_tags: Vec<Id>,
    pub(crate) bounding_points: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionedEntitySurface {
    pub(crate) tag: Id,
    pub(crate) parent_dim: i32,
    pub(crate) parent_tag: Id,
    pub(crate) partition_tags: Vec<Id>,
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
    pub(crate) physical_tags: Vec<Id>,
    pub(crate) bounding_curves: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionedEntityVolume {
    pub(crate) tag: Id,
    pub(crate) parent_dim: i32,
    pub(crate) parent_tag: Id,
    pub(crate) partition_tags: Vec<Id>,
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
    pub(crate) physical_tags: Vec<Id>,
    pub(crate) bounding_surfaces: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionedEntities {
    pub(crate) num_partitions: u64,
    pub(crate) ghosts: Vec<EntityGhost>,
    pub(crate) points: Vec<PartitionedEntityPoint>,
    pub(crate) curves: Vec<PartitionedEntityCurve>,
    pub(crate) surfaces: Vec<PartitionedEntitySurface>,
    pub(crate) volumes: Vec<PartitionedEntityVolume>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parametrisations {
    pub(crate) curves: Vec<ParametrisationsCurve>,
    pub(crate) surfaces: Vec<ParametrisationsSurface>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParametrisationsCurveNode {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) u: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParametrisationsCurve {
    pub(crate) tag: Id,
    pub(crate) curves: Vec<ParametrisationsCurveNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParametrisationsSurfaceNode {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) u: f64,
    pub(crate) v: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParametrisationsSurfaceTriangle(pub(crate) Id, pub(crate) Id, pub(crate) Id);

#[derive(Clone, Debug, PartialEq)]
pub struct ParametrisationsSurface {
    pub(crate) tag: Id,
    pub(crate) nodes: Vec<ParametrisationsSurfaceNode>,
    pub(crate) triangles: Vec<ParametrisationsSurfaceTriangle>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PeriodicLinkNode {
    pub(crate) tag: Id,
    pub(crate) master: Id,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PeriodicLink {
    pub(crate) dim: i32,
    pub(crate) tag: Id,
    pub(crate) master: Id,
    pub(crate) affine: Vec<f64>,
    pub(crate) corresponding: Vec<PeriodicLinkNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Periodic {
    pub(crate) links: Vec<PeriodicLink>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub(crate) format: Format,
    pub(crate) entities: Option<Entities>,
    pub(crate) partitioned_entities: Option<PartitionedEntities>,
    pub(crate) physical_names: Option<PhysicalNames>,
    pub(crate) nodes: Nodes,
    pub(crate) elements: Elements,
    pub(crate) periodic: Option<Periodic>,
    pub(crate) parametrisations: Option<Parametrisations>,
}

impl Mesh {
    pub fn to_legacy(self) -> mesh::Mesh {
        let Self {
            format,
            nodes: onodes,
            elements: oelements,
            entities: _,
            partitioned_entities: _,
            physical_names: _,
            periodic: _,
            parametrisations: _,
        } = self;
        let format =
            crate::format::Format::new(format.version.parse().unwrap(), format.file, format.size);
        let mut nodes: mesh::Nodes = Default::default();
        for node in onodes.entities.into_iter() {
            for (id, n) in node.node_tags.into_iter().zip(node.node_positions) {
                nodes.insert(id, crate::node::Node::new(n.x, n.y, n.z));
            }
        }
        let mut elements: mesh::Elements = Default::default();
        for element in oelements.entities.into_iter() {
            let tag = element.tag;
            for element in element.elements {
                elements.insert(tag, (0, 0, element.topology));
            }
        }
        mesh::Mesh::new(Some(format), nodes, elements)
    }
}

pub fn mesh<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Mesh, E> {
    all_consuming(mesh_not_consuming)(i)
}

pub(crate) fn mesh_not_consuming<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Mesh, E> {
    let (i, format) = terminated(
        map_parser(
            block("$MeshFormat\n", "\n$EndMeshFormat"),
            all_consuming(format),
        ),
        newline,
    )(i)?;

    let (i, physical_names) = opt(terminated(physical_names, newline))(i)?;
    let (i, entities) = opt(terminated(entities, newline))(i)?;
    let (i, partitioned_entities) = opt(terminated(partitioned_entities, newline))(i)?;

    let (i, nodes) = terminated(nodes, newline)(i)?;

    let (i, elements) = terminated(
        map_parser(
            block("$Elements\n", "$EndElements"),
            all_consuming(elements),
        ),
        newline,
    )(i)?;

    let (i, parametrisations) = opt(terminated(
        map_parser(
            block("$Parametrizations\n", "$EndParametrizations"),
            all_consuming(parametrisations),
        ),
        newline,
    ))(i)?;

    let (i, periodic) = opt(terminated(
        map_parser(
            block("$Periodic\n", "$EndPeriodic"),
            all_consuming(periodic),
        ),
        newline,
    ))(i)?;

    Ok((
        i,
        Mesh {
            format,
            physical_names,
            entities,
            partitioned_entities,
            nodes,
            elements,
            periodic,
            parametrisations,
        },
    ))
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
    let (i, (version, _, file, _, size)) = tuple((tag("4.1"), space1, i32, space1, i32))(i)?;

    let format = Format {
        version: version.to_owned(),
        file,
        size,
    };
    Ok((i, format))
}

fn physical_name<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PhysicalName, E> {
    let name_sep = "\"";
    map(
        tuple((
            i32,
            space1,
            id,
            space1,
            delimited(tag(name_sep), take_until(name_sep), tag(name_sep)),
        )),
        |(dimension, _, tag, _, name)| PhysicalName {
            dimension,
            tag,
            name: name.to_string(),
        },
    )(i)
}

fn physical_names<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PhysicalNames, E> {
    let (i, _) = terminated(tag("$PhysicalNames"), newline)(i)?;

    let (i, physical_names) =
        length_count(terminated(u64, newline), terminated(physical_name, newline))(i)?;
    let (i, _) = tag("$EndPhysicalNames")(i)?;

    Ok((i, physical_names))
}

fn point_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, EntityPoint, E> {
    let (i, (tag, _, x, _, y, _, z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        EntityPoint {
            tag,
            x,
            y,
            z,
            physical_tags,
        },
    ))
}

fn curve_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, EntityCurve, E> {
    let (i, (tag, _, min_x, _, min_y, _, min_z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, bounding_points) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        EntityCurve {
            tag,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            bounding_points,
        },
    ))
}

fn surface_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, EntitySurface, E> {
    let (i, (tag, _, min_x, _, min_y, _, min_z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, bounding_curves) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        EntitySurface {
            tag,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            bounding_curves,
        },
    ))
}

fn volume_tag<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, EntityVolume, E> {
    let (i, (tag, _, min_x, _, min_y, _, min_z, _)) =
        tuple((id, space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, bounding_surfaces) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        EntityVolume {
            tag,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            bounding_surfaces,
        },
    ))
}

fn entities<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Entities, E> {
    let (i, _) = terminated(tag("$Entities"), newline)(i)?;

    let (i, (npoints, _, ncurves, _, nsurfaces, _, nvolumes, _)) =
        tuple((u64, space1, u64, space1, u64, space1, u64, newline))(i)?;

    let (i, points) = count(terminated(point_tag, space0newline), npoints as usize)(i)?;
    let (i, curves) = count(terminated(curve_tag, space0newline), ncurves as usize)(i)?;
    let (i, surfaces) = count(terminated(surface_tag, space0newline), nsurfaces as usize)(i)?;
    let (i, volumes) = count(terminated(volume_tag, space0newline), nvolumes as usize)(i)?;

    let (i, _) = tag("$EndEntities")(i)?;

    Ok((
        i,
        Entities {
            points,
            curves,
            surfaces,
            volumes,
        },
    ))
}

fn ghost_entity<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, EntityGhost, E> {
    map(separated_pair(id, space1, i32), |(tag, partition)| {
        EntityGhost { tag, partition }
    })(i)
}

fn partitioned_point_tag<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PartitionedEntityPoint, E> {
    let (i, (tag, _, parent_dim, _, parent_tag, _)) =
        tuple((id, space1, i32, space1, id, space1))(i)?;

    let (i, partition_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, (_, x, _, y, _, z, _)) =
        tuple((space1, double, space1, double, space1, double, space1))(i)?;
    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        PartitionedEntityPoint {
            tag,
            parent_dim,
            parent_tag,
            partition_tags,
            x,
            y,
            z,
            physical_tags,
        },
    ))
}

fn partitioned_curve_tag<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PartitionedEntityCurve, E> {
    let (i, (tag, _, parent_dim, _, parent_tag, _)) =
        tuple((id, space1, i32, space1, id, space1))(i)?;
    let (i, partition_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, (_, min_x, _, min_y, _, min_z, _)) =
        tuple((space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, bounding_points) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        PartitionedEntityCurve {
            tag,
            parent_dim,
            parent_tag,
            partition_tags,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            bounding_points,
        },
    ))
}

fn partitioned_surface_tag<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PartitionedEntitySurface, E> {
    let (i, (tag, _, parent_dim, _, parent_tag, _)) =
        tuple((id, space1, i32, space1, id, space1))(i)?;
    let (i, partition_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, (_, min_x, _, min_y, _, min_z, _)) =
        tuple((space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, bounding_curves) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        PartitionedEntitySurface {
            tag,
            parent_dim,
            parent_tag,
            partition_tags,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            bounding_curves,
        },
    ))
}

fn partitioned_volume_tag<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PartitionedEntityVolume, E> {
    let (i, (tag, _, parent_dim, _, parent_tag, _)) =
        tuple((id, space1, i32, space1, id, space1))(i)?;
    let (i, partition_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, (_, min_x, _, min_y, _, min_z, _)) =
        tuple((space1, double, space1, double, space1, double, space1))(i)?;
    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, physical_tags) = length_count(u64, preceded(space1, id))(i)?;
    let (i, _) = space1(i)?;
    let (i, bounding_surfaces) = length_count(u64, preceded(space1, id))(i)?;

    Ok((
        i,
        PartitionedEntityVolume {
            tag,
            parent_dim,
            parent_tag,
            partition_tags,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            physical_tags,
            bounding_surfaces,
        },
    ))
}

fn partitioned_entities<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PartitionedEntities, E> {
    let (i, _) = terminated(tag("$PartitionedEntities"), newline)(i)?;

    let (i, num_partitions) = terminated(u64, newline)(i)?;

    let (i, ghosts) = length_count(terminated(u64, newline), terminated(ghost_entity, newline))(i)?;

    let (i, (npoints, _, ncurves, _, nsurfaces, _, nvolumes, _)) =
        tuple((u64, space1, u64, space1, u64, space1, u64, newline))(i)?;

    let (i, points) = count(
        terminated(partitioned_point_tag, space0newline),
        npoints as usize,
    )(i)?;
    let (i, curves) = count(
        terminated(partitioned_curve_tag, space0newline),
        ncurves as usize,
    )(i)?;
    let (i, surfaces) = count(
        terminated(partitioned_surface_tag, space0newline),
        nsurfaces as usize,
    )(i)?;
    let (i, volumes) = count(
        terminated(partitioned_volume_tag, space0newline),
        nvolumes as usize,
    )(i)?;

    let (i, _) = tag("$EndPartitionedEntities")(i)?;

    Ok((
        i,
        PartitionedEntities {
            num_partitions,
            ghosts,
            points,
            curves,
            surfaces,
            volumes,
        },
    ))
}

fn coordinate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Coordinate, E> {
    double(i)
}

fn id<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, node::Id, E> {
    i32(i)
}

fn entityblock<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, NodeEntity, E> {
    let (i, (dim, _, tag, _, parametric, _, num_nodes, _)) =
        tuple((i32, space1, id, space1, i32, space1, u64, newline))(i)?;
    let parametric = parametric == 1;

    let (i, node_tags) = count(terminated(id, newline), num_nodes as usize)(i)?;
    let (i, node_positions) = count(
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
            |(x, _, y, _, z, u, v, w)| Position { x, y, z, u, v, w },
        ),
        num_nodes as usize,
    )(i)?;

    Ok((
        i,
        NodeEntity {
            dim,
            tag,
            node_tags,
            node_positions,
        },
    ))
}

fn nodes<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Nodes, E> {
    let (i, _) = terminated(tag("$Nodes"), newline)(i)?;

    let (i, (num_ent_blocks, _, num_nodes, _, min_node, _, max_node, _)) =
        tuple((u64, space1, u64, space1, id, space1, id, newline))(i)?;

    let (i, entities) = count(entityblock, num_ent_blocks as usize)(i)?;

    let (i, _) = tag("$EndNodes")(i)?;

    Ok((
        i,
        Nodes {
            min_node,
            max_node,
            num_nodes,
            entities,
        },
    ))
}

fn element_parser<'a, E: ParseError<&'a str>>(
    typ: i32,
) -> impl FnMut(&'a str) -> IResult<&'a str, TaggedTopology, E> {
    move |i| {
        let (i, tag) = terminated(i32, space1)(i)?;
        let (i, topology) = match typ {
            1 => line(i),
            2 => triangle3(i),
            3 => quadrangle4(i),
            4 => tetrahedron4(i),
            5 => hexahedron8(i),
            6 => prism6(i),
            15 => point(i),
            x => unimplemented!("Element type {}", x),
        }?;

        Ok((i, TaggedTopology { tag, topology }))
    }
}

fn element_group<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ElementEntity, E> {
    let (i, (dim, _, tag, _, typ, _, num_elements_in_block, _)) =
        tuple((i32, space1, id, space1, i32, space1, u64, newline))(i)?;

    let (i, elements) = count(
        terminated(element_parser(typ), space0newline),
        num_elements_in_block as usize,
    )(i)?;

    Ok((i, ElementEntity { dim, tag, elements }))
}

fn elements<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Elements, E> {
    let (i, (num_entity_blocks, _, num_elements, _, min_tag, _, max_tag, _)) =
        tuple((u64, space1, u64, space1, id, space1, id, newline))(i)?;

    let (i, entities) = count(element_group, num_entity_blocks as usize)(i)?;

    Ok((
        i,
        Elements {
            min_tag,
            max_tag,
            num_elements,
            entities,
        },
    ))
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

fn prism6<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = terminated(id, space1)(i)?;
    let (i, x1) = terminated(id, space1)(i)?;
    let (i, x2) = terminated(id, space1)(i)?;
    let (i, x3) = terminated(id, space1)(i)?;
    let (i, x4) = terminated(id, space1)(i)?;
    let (i, x5) = id(i)?;

    Ok((i, Topology::Prism6(x0, x1, x2, x3, x4, x5)))
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

fn parametrisations_curve_node<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ParametrisationsCurveNode, E> {
    map(
        tuple((double, space1, double, space1, double, space1, double)),
        |(x, _, y, _, z, _, u)| ParametrisationsCurveNode { x, y, z, u },
    )(i)
}

fn parametrisations_curve<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ParametrisationsCurve, E> {
    // Undocumented: newline (should be space1?)
    let (i, (tag, _, num_curves, _)) = tuple((id, newline, u64, newline))(i)?;
    let (i, curves) = count(
        terminated(parametrisations_curve_node, newline),
        num_curves as usize,
    )(i)?;

    Ok((i, ParametrisationsCurve { tag, curves }))
}

fn parametrisations_surface_node<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ParametrisationsSurfaceNode, E> {
    let (i, (x, _, y, _, z, _)) = tuple((double, space1, double, space1, double, space1))(i)?;
    let (i, (u, _, v, _)) = tuple((double, space1, double, space1))(i)?;

    let (i, (max_x, _, max_y, _, max_z, _)) =
        tuple((double, space1, double, space1, double, space1))(i)?;

    let (i, (min_x, _, min_y, _, min_z)) = tuple((double, space1, double, space1, double))(i)?;

    Ok((
        i,
        ParametrisationsSurfaceNode {
            x,
            y,
            z,
            u,
            v,
            max_x,
            max_y,
            max_z,
            min_x,
            min_y,
            min_z,
        },
    ))
}

fn parametrisations_surface_triangle<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ParametrisationsSurfaceTriangle, E> {
    map(tuple((id, space1, id, space1, id)), |(i1, _, i2, _, i3)| {
        ParametrisationsSurfaceTriangle(i1, i2, i3)
    })(i)
}

fn parametrisations_surface<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ParametrisationsSurface, E> {
    let (i, (tag, _, num_nodes, _, num_triangles, _)) =
        tuple((id, newline, u64, space1, u64, newline))(i)?;

    let (i, nodes) = count(
        terminated(parametrisations_surface_node, newline),
        num_nodes as usize,
    )(i)?;

    let (i, triangles) = count(
        terminated(parametrisations_surface_triangle, newline),
        num_triangles as usize,
    )(i)?;

    Ok((
        i,
        ParametrisationsSurface {
            tag,
            nodes,
            triangles,
        },
    ))
}

fn parametrisations<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Parametrisations, E> {
    let (i, (num_curve_params, _, num_surface_params, _)) = tuple((u64, space1, u64, newline))(i)?;

    let (i, curves) = count(parametrisations_curve, num_curve_params as usize)(i)?;
    let (i, surfaces) = count(parametrisations_surface, num_surface_params as usize)(i)?;

    Ok((i, Parametrisations { curves, surfaces }))
}

fn periodic_link_node<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PeriodicLinkNode, E> {
    map(tuple((id, space1, id, newline)), |(tag, _, master, _)| {
        PeriodicLinkNode { tag, master }
    })(i)
}

fn periodic_link<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PeriodicLink, E> {
    let (i, (dim, _, tag, _, master, _)) = tuple((i32, space1, id, space1, id, newline))(i)?;

    let (i, affine) = terminated(length_count(u64, preceded(space1, double)), newline)(i)?;

    let (i, corresponding) = length_count(terminated(u64, newline), periodic_link_node)(i)?;

    Ok((
        i,
        PeriodicLink {
            dim,
            tag,
            master,
            affine,
            corresponding,
        },
    ))
}

fn periodic<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Periodic, E> {
    let (i, num_links) = terminated(u64, newline)(i)?;

    map(count(periodic_link, num_links as usize), |links| Periodic {
        links,
    })(i)
}

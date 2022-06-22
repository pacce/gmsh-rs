use {
    crate::{
        element::{self, Elementary, Physical, Topology},
        mesh::{self, Mesh},
        node::{self, Coordinate, Node},
    },
    nom::{
        bytes::complete,
        character::complete::{newline, space0},
        error::ParseError,
        multi,
        number::complete::double,
        IResult,
    },
    std::collections::HashMap,
};

pub fn mesh<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Mesh, E> {
    let (i, ns) = nodes(i)?;
    let (i, _) = newline(i)?;
    let (i, es) = elements(i)?;

    Ok((i, Mesh::new(None, ns, es)))
}

fn coordinate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Coordinate, E> {
    double(i)
}

fn id<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, node::Id, E> {
    let (i, id) = double(i)?;
    Ok((i, id as node::Id))
}

fn node<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (node::Id, Node), E> {
    let (i, id) = id(i)?;
    let (i, _) = space0(i)?;

    let (i, x) = coordinate(i)?;
    let (i, _) = space0(i)?;
    let (i, y) = coordinate(i)?;
    let (i, _) = space0(i)?;
    let (i, z) = coordinate(i)?;

    let (i, _) = newline(i)?;

    let node = Node::new(x, y, z);
    Ok((i, (id, node)))
}

fn nodes<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, mesh::Nodes, E> {
    let (i, _) = complete::tag("$NOD")(i)?;
    let (i, _) = newline(i)?;

    let (i, n) = double(i)?;
    let (i, _) = newline(i)?;

    let (i, ns) = multi::count(node, n as usize)(i)?;

    let (i, _) = complete::tag("$ENDNOD")(i)?;

    let mut nodes: mesh::Nodes = HashMap::new();
    for (id, node) in ns {
        nodes.insert(id, node);
    }

    Ok((i, nodes))
}

fn topology<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, i32, E> {
    let (i, n) = double(i)?;
    Ok((i, n as i32))
}

fn physical<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Physical, E> {
    let (i, n) = double(i)?;
    Ok((i, n as Physical))
}

fn elementary<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Elementary, E> {
    let (i, n) = double(i)?;
    Ok((i, n as Elementary))
}

fn element<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (element::Id, Physical, Elementary, Topology), E> {
    let (i, id) = id(i)?;
    let (i, _) = space0(i)?;

    let (i, t) = topology(i)?;
    let (i, _) = space0(i)?;

    let (i, p) = physical(i)?;
    let (i, _) = space0(i)?;

    let (i, e) = elementary(i)?;
    let (i, _) = space0(i)?;

    let (i, _) = double(i)?;
    let (i, _) = space0(i)?;

    let (i, topology) = match t {
        1 => line(i)?,
        2 => triangle3(i)?,
        3 => quadrangle4(i)?,
        4 => tetrahedron4(i)?,
        5 => hexahedron8(i)?,
        15 => point(i)?,
        _ => unimplemented!(),
    };

    let (i, _) = newline(i)?;

    Ok((i, (id, p, e, topology)))
}

fn elements<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, mesh::Elements, E> {
    let (i, _) = complete::tag("$ELM")(i)?;
    let (i, _) = newline(i)?;

    let (i, n) = double(i)?;
    let (i, _) = newline(i)?;

    let (i, es) = multi::count(element, n as usize)(i)?;

    let (i, _) = complete::tag("$ENDELM")(i)?;

    let mut elements: mesh::Elements = HashMap::new();
    for (id, p, e, t) in es {
        elements.insert(id, (p, e, t));
    }

    Ok((i, elements))
}

// Element parser

fn line<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x1) = id(i)?;

    Ok((i, Topology::Line2(x0, x1)))
}

fn triangle3<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x1) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x2) = id(i)?;

    Ok((i, Topology::Triangle3(x0, x1, x2)))
}

fn quadrangle4<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x1) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x2) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x3) = id(i)?;

    Ok((i, Topology::Quadrangle4(x0, x1, x2, x3)))
}

fn tetrahedron4<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x1) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x2) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x3) = id(i)?;

    Ok((i, Topology::Tetrahedron4(x0, x1, x2, x3)))
}

fn hexahedron8<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x1) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x2) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x3) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x4) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x5) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x6) = id(i)?;
    let (i, _) = space0(i)?;
    let (i, x7) = id(i)?;

    Ok((i, Topology::Hexahedron8(x0, x1, x2, x3, x4, x5, x6, x7)))
}

fn point<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Topology, E> {
    let (i, x0) = id(i)?;

    Ok((i, Topology::Point1(x0)))
}

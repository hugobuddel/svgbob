use crate::{
    buffer::{
        fragment::PolygonTag::{
            ArrowBottom, ArrowBottomLeft, ArrowBottomRight, ArrowLeft, ArrowRight, ArrowTop,
            ArrowTopLeft, ArrowTopRight, DiamondBullet,
        },
        Cell, CellGrid, Settings,
    },
    fragment::{arc, broken_line, circle, line, polygon, rect},
    Fragment, Property,
    Signal::{self, Medium, Strong, Weak},
};
use lazy_static::lazy_static;
use std::{collections::BTreeMap, sync::Arc};

lazy_static! {

    /// ```ignore
    ///      0 1 2 3 4           B C D
    ///     0┌─┬─┬─┬─┐        A┌─┬─┬─┬─┐E
    ///     1├─┼─┼─┼─┤         │ │ │ │ │
    ///     2├─┼─┼─┼─┤        F├─G─H─I─┤J
    ///     3├─┼─┼─┼─┤         │ │ │ │ │
    ///     4├─┼─┼─┼─┤        K├─L─M─N─┤O
    ///     5├─┼─┼─┼─┤         │ │ │ │ │
    ///     6├─┼─┼─┼─┤        P├─Q─R─S─┤T
    ///     7├─┼─┼─┼─┤         │ │ │ │ │
    ///     8└─┴─┴─┴─┘        U└─┴─┴─┴─┘Y
    /// ```                      V W X

    pub static ref ASCII_PROPERTIES: BTreeMap<char, Property> = {
        #![allow(unused)]

        let cell = Cell::new(0,0);

        let a = CellGrid::a();
        let b = CellGrid::b();
        let c = CellGrid::c();
        let d = CellGrid::d();
        let e = CellGrid::e();
        let f = CellGrid::f();
        let g = CellGrid::g();
        let h = CellGrid::h();
        let i = CellGrid::i();
        let j = CellGrid::j();
        let k = CellGrid::k();
        let l = CellGrid::l();
        let m = CellGrid::m();
        let n = CellGrid::n();
        let o = CellGrid::o();
        let p = CellGrid::p();
        let q = CellGrid::q();
        let r = CellGrid::r();
        let s = CellGrid::s();
        let t = CellGrid::t();
        let u = CellGrid::u();
        let v = CellGrid::v();
        let w = CellGrid::w();
        let x = CellGrid::x();
        let y = CellGrid::y();

        /// cellgrids that have no names
        /// just name them with coordinate locations
        let _01 = CellGrid::point(0, 1);
        let _11 = CellGrid::point(1, 1);
        let _21 = CellGrid::point(2, 1);
        let _31 = CellGrid::point(3, 1);
        let _41 = CellGrid::point(4, 1);
        let _03 = CellGrid::point(0, 3);
        let _13 = CellGrid::point(1, 3);
        let _23 = CellGrid::point(2, 3);
        let _33 = CellGrid::point(3, 3);
        let _43 = CellGrid::point(4, 3);
        let _05 = CellGrid::point(0, 5);
        let _15 = CellGrid::point(1, 5);
        let _25 = CellGrid::point(2, 5);
        let _35 = CellGrid::point(3, 5);
        let _45 = CellGrid::point(4, 5);
        let _07 = CellGrid::point(0, 7);
        let _17 = CellGrid::point(1, 7);
        let _27 = CellGrid::point(2, 7);
        let _37 = CellGrid::point(3, 7);
        let _47 = CellGrid::point(4, 7);

        let unit1 = Cell::unit(1); // 0.25
        let unit2 = Cell::unit(2); // 0.5
        let unit3 = Cell::unit(3); // 0.75
        let unit4 = Cell::unit(4); // 1.0
        let unit5 = Cell::unit(5); // 1.25
        let unit6 = Cell::unit(6); // 1.5
        let unit7 = Cell::unit(7); // 1.75
        let unit8 = Cell::unit(8); // 2.0

        // in between 1 and 2
        let between1_2 = (unit1 + unit2) / 2.0; // 0.375

        /// char, default fragments, conditional fragments
        let map: Vec<(
            char,
            Vec<(Signal, Vec<Fragment>)>,
            Arc<dyn Fn(&Settings, &Property, &Property, &Property, &Property, &Property, &Property, &Property, &Property) -> Vec<(bool, Vec<Fragment>)> + Sync + Send >,
        )> = vec![
            ///////////////
            // all letters and digits, necessary for .is_alphanumeric() to work
            ///////////////
            ('0',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('1',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('2',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('3',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('4',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('5',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('6',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('7',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('8',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('9',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('a',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('b',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('c',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('d',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('e',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('f',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('g',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('h',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('i',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('j',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('k',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('l',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('m',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('n',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            // ('o',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('p',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('q',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('r',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('s',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('t',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('u',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            // ('v',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('w',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('x',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('y',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('z',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('A',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('B',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('C',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('D',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('E',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('F',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('G',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('H',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('I',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('J',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('K',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('L',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('M',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('N',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            // ('O',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('P',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('Q',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('R',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('S',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('T',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('U',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            // ('V',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('W',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            // ('X',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('Y',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),
            ('Z',vec![],Arc::new(move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {vec![]})),

            ///////////////
            // dash -
            ///////////////
            (
                '-',
                vec![
                    (Strong, vec![line(k, o)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            // ensure that the dash is not in the middle of a word, e.g.
                            // the dashes in "e-mail" and "2019-10-31" should not become lines
                            // TODO: allow 'o-o' to become line?
                            (!left.ch.is_alphanumeric() || !right.ch.is_alphanumeric(), vec![line(k,o)]),

                            /*
                            //   |
                            //  .-.
                            //   |
                            (settings.enhance_circuitries && left.is('.') && right.is('.') && top.line_strongly_overlap(r,w) && bottom.line_strongly_overlap(c,h),
                                vec![arc(o,k,unit4),line(c,w)]
                            ),
                            */
                        ]
                    }
                )
            ),
            ///////////////
            // tilde ~
            ///////////////
            (
                '~',
                vec![
                    (Strong, vec![broken_line(k, o)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            (true, vec![broken_line(k, o)]),
                        ]
                    }
                )
            ),
            ////////////////////
            // vertical line |
            ////////////////////
            (
                '|',
                vec![
                    (Strong, vec![line(c,w)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            (!bottom_left.is('/') && !bottom_right.is('\\') && !top_left.is('\\') && !top_right.is('/'), vec![line(c,w)]),
                            //   _
                            //  |
                            (top_right.line_overlap(u, v), vec![line(c,e)]),
                            // _
                            //  |
                            (top_left.line_overlap(x, y), vec![line(a,c)]),
                            //  |_
                            (right.line_overlap(u,v), vec![line(w,y)]),
                            //  _|
                            (left.line_overlap(x,y), vec![line(u,w)]),
                            //  |-
                            (right.line_strongly_overlap(k,l), vec![line(m,o)]),
                            //  -|
                            (left.line_strongly_overlap(n,o), vec![line(k,m)]),
                            // TODO: restrict lef, right, bottom, top_right, is not connecting to
                            // here
                            //    |
                            //   /
                            (bottom_left.line_overlap(e,u), vec![line(c,m),line(m,u)]),
                            // TODO: restrict left, right, bottom, top_left, top_right
                            //   |
                            //    \
                            (bottom_right.line_overlap(a,y), vec![line(c,m), line(m,y)]),
                            // TODO: restrict left, right, top, bottom_left, bottom_right
                            //    \ /
                            //     |
                            (top_left.line_overlap(a,y) && top_right.line_overlap(e,u), vec![line(a,m),line(m,w),line(m,e)]),
                        ]
                    }
                )
            ),
            ////////////////////
            // exclamation bang !
            ////////////////////
            (
                '!',
                vec![
                    (Strong, vec![broken_line(c,w)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            (top.line_strongly_overlap(r,w) || bottom.line_strongly_overlap(c,h), vec![broken_line(c,w)]),
                        ]
                    }
                )
            ),
            ////////////////////
            // colon :
            ////////////////////
            (
                ':',
                vec![
                    (Strong, vec![broken_line(c,w)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            (top.line_strongly_overlap(r,w) || bottom.line_strongly_overlap(c,h), vec![broken_line(c,w)]),
                        ]
                    }
                )
            ),
            /////////////////////////
            // plus cross +
            /////////////////////////
            (
                '+',
                vec![
                    (Medium, vec![line(c,w),line(k,o)]),
                    (Weak, vec![line(a,y), line(u,e)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            //  |
                            //  +
                            (top.line_overlap(r,w), vec![line(c, m)]),
                            //  +
                            //  |
                            (bottom.line_overlap(c,h), vec![line(m,w)]),
                            // -+
                            (left.line_overlap(n,o), vec![line(k,m)]),
                            //  +-
                            (right.line_overlap(k,l), vec![line(m,o)]),

                            // .+
                            (left.line_weakly_overlap(n,o), vec![line(k,m)]),
                            //  +.
                            (right.line_weakly_overlap(k,l), vec![line(m,o)]),

                            // \
                            //  +
                            (top_left.line_overlap(s,y), vec![line(a, m)]),
                            //  +
                            //   \
                            (bottom_right.line_overlap(a,g), vec![line(m, y)]),
                            //   /
                            //  +
                            (top_right.line_overlap(q,u), vec![line(m, e)]),
                            //   +
                            //  /
                            (bottom_left.line_overlap(e,i), vec![line(m, u)]),
                        ]
                    }
                )
            ),
            /////////////////////////
            // letter X
            /////////////////////////
            (
                'X',
                vec![
                    (Strong, vec![line(a,y), line(u,e)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            /*
                            // \    X
                            //  X    \
                            (top_left.line_overlap(s,y) || bottom_right.line_overlap(a,g), vec![line(a, y)]),
                            //   /   X
                            //  X   /
                            (top_right.line_overlap(q,u) || bottom_left.line_overlap(e,i), vec![line(e, u)]),
                            */

                            //   -x
                            (left.line_strongly_overlap(m,o), vec![line(m,k)]),
                            //   x-
                            (right.line_strongly_overlap(k,l), vec![line(m,o)]),
                            //   |
                            //   x
                            (top.line_strongly_overlap(r,w), vec![line(m,c)]),
                            //   x
                            //   |
                            (bottom.line_strongly_overlap(c,h), vec![line(m,w)]),
                            //    \
                            //     x
                            (top_left.line_strongly_overlap(s,y), vec![line(m,a)]),
                            //     /
                            //    x
                            (top_right.line_strongly_overlap(u,q), vec![line(m,e)]),
                            //     x
                            //    /
                            (bottom_left.line_strongly_overlap(e,i), vec![line(m,u)]),
                            //     x
                            //      \
                            (bottom_right.line_strongly_overlap(a,g), vec![line(m,y)]),
                        ]
                    }
                )
            ),
            /////////////////////////
            // asterisk *
            /////////////////////////
            (
                '*',
                vec![
                    (Strong, vec![circle(m, unit2, true)]),
                    (Medium, vec![line(c,w),line(k,o)]),
                    (Weak, vec![line(a,y), line(u,e)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            (true, vec![circle(m,unit2, true)]),
                            //   -*
                            (left.line_strongly_overlap(m,o), vec![line(m,k)]),
                            //   *-
                            (right.line_strongly_overlap(k,l), vec![line(m,o)]),
                            //   |
                            //   *
                            (top.line_strongly_overlap(r,w), vec![line(m,c)]),
                            //   *
                            //   |
                            (bottom.line_strongly_overlap(c,h), vec![line(m,w)]),
                            //    \
                            //     *
                            (top_left.line_strongly_overlap(s,y), vec![line(m,a)]),
                            //     /
                            //    *
                            (top_right.line_strongly_overlap(u,q), vec![line(m,e)]),
                            //     *
                            //    /
                            (bottom_left.line_strongly_overlap(e,i), vec![line(m,u)]),
                            //     *
                            //      \
                            (bottom_right.line_strongly_overlap(a,g), vec![line(m,y)]),
                        ]
                    }
                )
            ),

            /////////////////////////
            // hash pound #
            /////////////////////////
            (
                '#',
                vec![
                    (Strong, vec![rect(f,t,true, false)]),
                    (Medium, vec![line(c,w),line(k,o)]),
                    (Weak, vec![line(a,y), line(u,e)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            (top.line_overlap(r,w) || bottom.line_overlap(c,h)
                                || left.line_overlap(n,o) || right.line_overlap(k,l),
                             vec![rect(f,t, true, false)]),

                            (top_left.line_overlap(s,y)|| bottom_right.line_overlap(a,g)
                              || bottom_left.line_overlap(u,q) || top_right.line_overlap(e,i),
                             vec![polygon(vec![k,h,o,r,k],true, vec![DiamondBullet])]),
                        ]
                    }
                )
            ),
            /////////////////////////
            // small letter o
            /////////////////////////
            (
                'o',
                vec![
                    (Medium, vec![circle(m, unit1, false)]),
                    (Medium, vec![line(k,o)]),
                    (Weak, vec![line(c,w)]),
                    (Weak, vec![line(a,y), line(u,e)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            //  must have at least one connection
                            //   \|/
                            //   -o-
                            //   /|\
                            (top.line_strongly_overlap(r,w) || bottom.line_strongly_overlap(c,h)
                                || left.line_strongly_overlap(n,o) || right.line_strongly_overlap(k,l)
                                || top_left.line_strongly_overlap(s,y)|| bottom_right.line_strongly_overlap(a,g)
                              || bottom_left.line_strongly_overlap(u,q) || top_right.line_strongly_overlap(e,i),
                                vec![circle(m,unit1,false)]),
                            //   -o
                            (left.line_strongly_overlap(m,o), vec![line(m,k)]),
                            //   o-
                            (right.line_strongly_overlap(k,l), vec![line(m,o)]),
                            //   |
                            //   o
                            (top.line_strongly_overlap(r,w), vec![line(m,c)]),
                            //   o
                            //   |
                            (bottom.line_strongly_overlap(c,h), vec![line(m,w)]),
                            //    \
                            //     o
                            (top_left.line_strongly_overlap(s,y), vec![line(m,a)]),
                            //     /
                            //    o
                            (top_right.line_strongly_overlap(u,q), vec![line(m,e)]),
                            //     o
                            //    /
                            (bottom_left.line_strongly_overlap(e,i), vec![line(m,u)]),
                            //     o
                            //      \
                            (bottom_right.line_strongly_overlap(a,g), vec![line(m,y)]),
                        ]
                    }
                )
            ),
            /////////////////////////
            // big letter O
            /////////////////////////
            (
                'O',
                vec![
                    (Medium, vec![circle(m, unit2, false)]),
                    (Medium, vec![line(k,o)]),
                    (Weak, vec![line(c,w)]),
                    (Weak, vec![line(a,y), line(u,e)]),
                ],
                Arc::new(
                    move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            //  must have at least one connection
                            //   \|/
                            //   -O-
                            //   /|\
                            (top.line_strongly_overlap(r,w) || bottom.line_strongly_overlap(c,h)
                                || left.line_strongly_overlap(n,o) || right.line_strongly_overlap(k,l)
                                || top_left.line_strongly_overlap(s,y)|| bottom_right.line_strongly_overlap(a,g)
                              || bottom_left.line_strongly_overlap(u,q) || top_right.line_strongly_overlap(e,i), vec![circle(m,unit2,false)]),
                            //   -O
                            (left.line_strongly_overlap(m,o), vec![line(m,k)]),
                            //   O-
                            (right.line_strongly_overlap(k,l), vec![line(m,o)]),
                            //   |
                            //   O
                            (top.line_strongly_overlap(r,w), vec![line(m,c)]),
                            //   O
                            //   |
                            (bottom.line_strongly_overlap(c,h), vec![line(m,w)]),
                            //    \
                            //     O
                            (top_left.line_strongly_overlap(s,y), vec![line(m,a)]),
                            //     /
                            //    O
                            (top_right.line_strongly_overlap(u,q), vec![line(m,e)]),
                            //     O
                            //    /
                            (bottom_left.line_strongly_overlap(e,i), vec![line(m,u)]),
                            //     O
                            //      \
                            (bottom_right.line_strongly_overlap(a,g), vec![line(m,y)]),
                        ]
                    }
                )
            ),
            ////////////////////
            // underscore _
            ////////////////////
            (
                '_',
                vec![
                    (Strong, vec![line(u, y)])
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                             (true, vec![line(u,y)]),
                             //   /_
                             (left.line_strongly_overlap(e,u), vec![line(u, cell.left().u())]),
                             //   _\
                             (right.line_strongly_overlap(a,y), vec![line(y, cell.right().y())]),
                        ]}

                    )
            ),
            //////////////////////
            // dot period .
            //////////////////////
            (
                '.',
                vec![
                    (Medium, vec![line(m,w)]), // connects down
                    (Weak, vec![line(m,k)]), // connects left
                    (Weak, vec![line(m,o)]), // connects right
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            // .
                            // |
                            (bottom.line_strongly_overlap(c,h), vec![line(r,w)]),
                            //   .
                            //  / \
                            (bottom_left.line_strongly_overlap(e,i) && bottom_right.line_strongly_overlap(a,g), vec![line(m,u), line(m,y)]),
                            //  .-
                            //  |
                            (right.line_overlap(k,l) && bottom.line_overlap(c,h), vec![arc(o,r,unit2), line(r,w)]),
                            //   .-
                            //  |
                            (right.line_overlap(k,l) && bottom_left.line_overlap(c,h), vec![arc(m,cell.bottom_left().c(),unit4), line(m,o)]),
                            // -.
                            //  |
                            (left.line_overlap(n,o) && bottom.line_overlap(c,h), vec![arc(r,k,unit2), line(r,w)]),
                            // -.
                            //   |
                            //  exemption that bottom right is not a backquote
                            (!bottom_right.is('`') && left.line_overlap(n,o) && bottom_right.line_overlap(c,h), vec![arc(cell.bottom_right().c(),m,unit4), line(k,m)]),
                            //     .-
                            //    /
                            (right.line_overlap(k,l) && bottom_left.line_overlap(e,i), vec![arc(o, q, unit4), line(q, u)]),
                            //     .-
                            //      \
                            (right.line_overlap(k,l) && bottom_right.line_overlap(a,g) , vec![arc(o, s, between1_2), line(s, y)]),
                            //     -.
                            //       \
                            (left.line_overlap(n,o) && bottom_right.line_overlap(a,g), vec![arc(s, k, unit4), line(s, y)]),
                            //     -.
                            //     /
                            (left.line_overlap(n,o) && bottom_left.line_overlap(e,i), vec![arc(q, k, between1_2), line(u, q)]),
                            //   .
                            //  (
                            (bottom_left.arcs_to(e,y), vec![arc(o, q, unit4), line(q, u)]),
                            //  .
                            //   )
                            (bottom_right.arcs_to(u,a),vec![arc(s, k, unit4), line(s, y)]),

                            //   _.-
                            (left.line_overlap(u,y) && right.line_overlap(k,o), vec![line(u,o)]),
                            //   -._
                            (left.line_overlap(k,o) && right.line_overlap(u,y), vec![line(k,y)]),

                            // `.
                            //   `
                            (left.is('`') && bottom_right.is('`'), vec![broken_line(cell.left().c(), cell.bottom_right().c())]),
                            //   .'
                            //  '
                            (right.is('\'') && bottom_left.is('\''),vec![broken_line(cell.right().c(), cell.bottom_left().c())]),
                            // '.   `.
                            //   \    \
                            ((left.is('\'')||left.is('`')) && bottom_right.line_overlap(a,m),vec![arc(y, cell.left().a(), unit8 * 2.0)]),
                            //   .'
                            //  /
                            (right.is('\'') && bottom_left.line_overlap(e,m), vec![arc(cell.right().e(), u, unit8 * 2.0)]),
                            // TODO: restrict left, right, bottom, top_right, is not connecting to here
                            //     |
                            //     .
                            //    /
                            (top.line_overlap(m,w) && bottom_left.line_overlap(e,m), vec![arc(q,h,unit8), line(c,h), line(q,u)]),
                            // TODO: restrict left, right, bottom,top, is not connecting to here
                            //      /
                            //     .
                            //    /
                            (top_right.line_overlap(m,u) && bottom_left.line_overlap(e,m), vec![line(u,e)]),
                            // TODO: restrict left, right, bottom, top_left, does not connect to
                            // here
                            //   |
                            //   .
                            //    \
                            (top.line_overlap(m,w) && bottom_right.line_overlap(a,m), vec![line(c,h), arc(h,s,unit8), line(s,y)]),
                        ]}
                    )
            ),
            //////////////////////
            // comma ,
            //////////////////////
            (
                ',',
                vec![
                    (Medium, vec![line(m,r)]),
                    (Weak, vec![line(m,k)]), // connects left
                    (Weak, vec![line(m,o)]), // connects right
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            //  ,-
                            //  |
                            (right.line_overlap(k,l) && bottom.line_overlap(c,h), vec![arc(o,r,unit2), line(r,w)]),
                            //     ,-
                            //    /
                            (right.line_overlap(k,l) && bottom_left.line_overlap(e,i), vec![arc(o, q, unit4), line(q, u)]),
                            //   ,
                            //  (
                            (bottom_left.arcs_to(e,y), vec![arc(o, q, unit4), line(q, u)]),
                        ]}
                    )
            ),
            //////////////////////
            // single quote '
            //////////////////////
            (
                '\'',
                vec![
                    (Medium, vec![line(c,h)]), //connects top
                    (Weak, vec![line(m,k)]), // connects left
                    (Weak, vec![line(m,o)]), // connects right
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            //  |
                            //  '
                            (top.line_strongly_overlap(m,w), vec![line(c,h)]),
                            //  |
                            //  '-
                            (right.line_overlap(k,l) && top.line_overlap(r,w), vec![arc(h,o, unit2), line(c,h)]),
                            //  |
                            //   '-
                            (right.line_overlap(k,l) && top_left.line_overlap(r,w), vec![arc(cell.top_left().w(),m, unit4), line(m,o)]),
                            //   |
                            //  -'
                            (left.line_overlap(n,o) && top.line_overlap(r,w), vec![arc(k,h, unit2), line(c,h)]),
                            //    |
                            //  -'
                            (left.line_overlap(n,o) && top_right.line_overlap(r,w), vec![arc(m,cell.top_right().w(), unit4), line(k,m)]),

                            //    \
                            //     '-
                            (top_left.line_overlap(s,y) && right.line_overlap(k,l), vec![line(a, g), arc(g, o, unit4)]),
                            //      /
                            //     '-
                            (top_right.line_overlap(u,q) && right.line_overlap(k,l), vec![line(e, i), arc(i, o, between1_2)]),
                            //       /
                            //     -'
                            (top_right.line_overlap(u,q) && left.line_overlap(n,o), vec![arc(k, i, unit4), line(i, e)]),
                            //     \
                            //     -'
                            (top_left.line_overlap(s,y) && left.line_overlap(n,o), vec![arc(k, g, between1_2), line(g, a)]),
                            //     \ /
                            //      '
                            (top_left.line_overlap(s,y) && top_right.line_overlap(u,q), vec![line(a, m), line(m, e)]),
                            //  (
                            //   '
                            (top_left.arcs_to(e,y), vec![line(a, g), arc(g, o, unit4)]),
                            //   )
                            //  '
                            (top_right.arcs_to(u,a), vec![arc(k, i, unit4), line(i, e)]),
                            //    _
                            //  -'
                            (left.line_overlap(k,o) && top_right.line_overlap(u,y), vec![line(k,e)]),
                            //  _
                            //   '-
                            (top_left.line_overlap(u,y) && right.line_overlap(k,o), vec![line(a,o)]),

                            //    .
                            //  .'
                            (left.is('.') && top_right.is('.'), vec![broken_line(cell.left().m(), cell.top_right().m())]),
                            //  .
                            //   '.
                            (right.is('.') && top_left.is('.'), vec![broken_line(cell.top_left().m(), cell.right().m())]),
                            //    /
                            //  .'
                            (left.is('.') && top_right.is('/'),vec![arc(cell.left().u(), e, unit8 * 2.0)]),
                        ]}
                    )
            ),
            //////////////////////
            // other single quote ’
            //////////////////////
            (
                '’',
                vec![
                    (Medium, vec![line(c,h)]), //connects top
                    (Weak, vec![line(m,k)]), // connects left
                    (Weak, vec![line(m,o)]), // connects right
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            //  |
                            //  ’-
                            (right.line_overlap(k,l) && top.line_overlap(r,w), vec![arc(h,o, unit2), line(c,h)]),
                            //   |
                            //  -’
                            (left.line_overlap(n,o) && top.line_overlap(r,w), vec![arc(k,h, unit2), line(c,h)]),

                            //    \
                            //     ’-
                            (top_left.line_overlap(s,y) && right.line_overlap(k,l), vec![line(a, g), arc(g, o, unit4)]),
                            //      /
                            //     ’-
                            (top_right.line_overlap(u,q) && right.line_overlap(k,l), vec![line(e, i), arc(i, o, between1_2)]),
                            //       /
                            //     -’
                            (top_right.line_overlap(u,q) && left.line_overlap(n,o), vec![arc(k, i, unit4), line(i, e)]),
                            //     \
                            //     -’
                            (top_left.line_overlap(s,y) && left.line_overlap(n,o), vec![arc(k, g, between1_2), line(g, a)]),
                            //     \ /
                            //      ’
                            (top_left.line_overlap(s,y) && top_right.line_overlap(u,q), vec![line(a, m), line(m, e)]),
                            //  (
                            //   ’
                            (top_left.arcs_to(e,y), vec![line(a, g), arc(g, o, unit4)]),
                            //   )
                            //  ’
                            (top_right.arcs_to(u,a), vec![arc(k, i, unit4), line(i, e)]),
                            //    _
                            //  -’
                            (left.line_overlap(k,o) && top_right.line_overlap(u,y), vec![line(k,e)]),
                            //  _
                            //   ’-
                            (top_left.line_overlap(u,y) && right.line_overlap(k,o), vec![line(a,o)]),

                            //    .
                            //  .’
                            (left.is('.') && top_right.is('.'), vec![broken_line(cell.left().m(), cell.top_right().m())]),
                            //  .
                            //   ’.
                            (right.is('.') && top_left.is('.'), vec![broken_line(cell.top_left().m(), cell.right().m())]),
                        ]}
                    )
            ),
            //////////////////////
            // backtick / backquote  `
            //////////////////////
            (
                '`',
                vec![
                    (Medium, vec![line(c,m)]),
                    (Weak, vec![line(m,k)]), // connects left
                    (Weak, vec![line(m,o)]), // connects right
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            //  |
                            //  `-
                            (right.line_overlap(k,l) && top.line_overlap(r,w), vec![arc(h,o, unit2), line(c,h)]),
                            //    \
                            //     `-
                            (top_left.line_overlap(s,y) && right.line_overlap(k,l), vec![line(a, g), arc(g, o, unit4)]),
                            //  (
                            //   `
                            (top_left.arcs_to(e,y), vec![line(a, g), arc(g, o, unit4)]),
                            //  _
                            //   `-
                            (top_left.line_overlap(u,y) && right.line_overlap(k,o), vec![line(a,o)]),
                            // .
                            //  `.
                            (top_left.is('.') && right.is('.'), vec![broken_line(cell.top_left().m(), cell.right().m())]),
                            //  \
                            //   `.
                            (top_left.is('\\') && right.is('.') ,vec![arc(a,cell.right().y(),unit8 * 2.0)]),
                            //  ,
                            //  `-
                            (top.is(',') && right.line_overlap(k,l), vec![arc(h,o, unit2), line(c,h)]),
                        ]}
                    )
            ),
            //////////////////////
            // forward slash / slant line
            //////////////////////
            (
                '/',
                vec![
                    (Strong, vec![line(u,e)]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                             (!bottom.line_strongly_overlap(c,h), vec![line(u,e)]),
                             //    /-
                             (settings.enhance_circuitries && right.line_strongly_overlap(k,l), vec![line(m,o)]),
                             //    -/
                             (settings.enhance_circuitries && left.line_strongly_overlap(n,o), vec![line(m,k)]),
                             //    /
                             //    |
                             (settings.enhance_circuitries && bottom.line_strongly_overlap(c,h), vec![line(e,m),line(m,w)]),
                        ]}
                    )
            ),
            //////////////////////
            // backward slash \
            //////////////////////
            (
                '\\',
                vec![
                    (Strong, vec![line(a,y)]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                (!bottom.is('|'), vec![line(a,y)]),
                                /*
                                //  .
                                //   \
                                //    |
                                (top_left.is('.') && bottom_right.is('|'),  vec![arc(cell.bottom_right().w(),a, unit8 * 4.0)]),
                                */

                                //    \
                                //    |
                                (bottom.line_overlap(c,m), vec![line(a,m),line(m,w)]),
                                //    \-
                                (settings.enhance_circuitries && right.line_strongly_overlap(k,l), vec![line(m,o)]),
                                //    -\
                                (settings.enhance_circuitries && left.line_strongly_overlap(n,o), vec![line(m,k)]),
                            ]
                        }
                    )
            ),
            ////////////////////////
            // open parenthesis (
            ////////////////////////
            ('(',
                vec![
                  (Medium, vec![arc(e,y, unit8)]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                (!top.line_overlap(r,w) && !bottom.line_overlap(c,h),  vec![arc(e,y,unit8)]),
                                //   |
                                //   (
                                //   |
                                (settings.enhance_circuitries && top.line_overlap(r,w) && bottom.line_overlap(c,h), vec![arc(c,w,unit6)]),
                                //   -(-
                                (settings.enhance_circuitries && left.line_overlap(m,o) && right.line_overlap(k,l), vec![line(k,o)]),
                            ]
                        }
                    )
            ),
            ////////////////////////
            // close parenthesis )
            ////////////////////////
            (')',
                vec![
                  (Medium, vec![arc(u,a, unit8)]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                (!top.line_overlap(r,w) && !bottom.line_overlap(c,h), vec![arc(u,a,unit8)]),
                                //   |
                                //   )
                                //   |
                                (settings.enhance_circuitries && top.line_overlap(r,w) && bottom.line_overlap(c,h), vec![arc(w,c,unit6)]),
                                //   -)-
                                (settings.enhance_circuitries && left.line_overlap(m,o) && right.line_overlap(k,l), vec![line(k,o)]),
                            ]
                        }
                    )
            ),
            ////////////////////////
            // big letter V
            ////////////////////////
            ('V',
                vec![
                  (Medium, vec![polygon(vec![f,j,w], true, vec![ArrowBottom])]),
                  (Weak, vec![line(m,w)]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                //  |
                                //  V
                               (top.line_overlap(r,w), vec![polygon(vec![f,j,w], true, vec![ArrowBottom])]),
                               // \
                               //  V
                               //  TODO: use arrow function which alias to a polygon
                               (top_left.line_overlap(s,y), vec![polygon(vec![f,s,_21], true, vec![ArrowBottomRight])]),
                               //    /
                               //   V
                               (top_right.line_overlap(u,q), vec![polygon(vec![j,q,_21], true, vec![ArrowBottomLeft])]),
                               //  `.
                               //    V
                               (top_left.is('.'), vec![polygon(vec![f,o,c], true, vec![ArrowBottomRight])]),
                               //     .'
                               //    V
                               (top_right.is('.'), vec![polygon(vec![j,k,c], true, vec![ArrowBottomLeft])]),
                               //    V
                               //    |
                               (bottom.line_overlap(c,h), vec![line(a,w), line(w,e)])
                            ]
                        }
                    )
            ),
            ////////////////////////
            // letter v
            ////////////////////////
            ('v',
                vec![
                  (Medium, vec![polygon(vec![f,j,w], true, vec![ArrowBottom])]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                // |
                                // v
                               (top.line_overlap(r,w), vec![polygon(vec![f,j,w], true, vec![ArrowBottom])]),
                               // \
                               //  v
                               (top_left.line_overlap(s,y), vec![polygon(vec![f,s,_21], true, vec![ArrowBottomRight])]),
                               //    /
                               //   v
                               (top_right.line_overlap(u,q), vec![polygon(vec![j,q,_21], true, vec![ArrowBottomLeft])]),
                               //  `.
                               //    v
                               (top_left.is('.'), vec![polygon(vec![f,o,c], true, vec![ArrowBottomRight])]),
                               //     .'
                               //    v
                               (top_right.is('.'), vec![polygon(vec![j,k,c], true, vec![ArrowBottomLeft])]),
                               //    v
                               //    |
                               (bottom.line_overlap(c,h) && !top.line_overlap(r,w), vec![line(a,w), line(w,e)])
                            ]
                        }
                    )
            ),
            ////////////////////////
            // caret ^
            ////////////////////////
            ('^',
                vec![
                  (Medium, vec![polygon(vec![p,c,t], true, vec![ArrowTop])]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                               //   ^
                               //   |
                               (bottom.line_overlap(c,h), vec![polygon(vec![p,c,t], true, vec![ArrowTop])]),
                               //  ^
                               //   \
                               (bottom_right.line_overlap(a,g) &&!bottom_left.is('/'), vec![polygon(vec![t,g,_27], true, vec![ArrowTopLeft])]),
                               //   ^
                               //  /
                               (bottom_left.line_overlap(e,i) && !bottom_right.is('\\'), vec![polygon(vec![p,i,_27], true, vec![ArrowTopRight])]),
                               //   |
                               //   ^
                               (top.line_overlap(r,w) && !bottom.line_overlap(c,h), vec![line(c,u), line(c,y)]),
                               //   ^
                               //  / \
                               (bottom_left.is('/') && bottom_right.is('\\'), vec![line(c,u), line(c,y)])
                            ]
                        }
                    )
            ),
            ////////////////////////
            // greater than >
            ////////////////////////
            ('>',
                vec![
                  (Medium, vec![polygon(vec![f,o,p], true, vec![ArrowRight])]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                //   --->
                               (left.line_overlap(n,o), vec![polygon(vec![f,o,p], true, vec![ArrowRight])]),
                               //   >----
                               (right.line_overlap(k,l) && !left.line_overlap(n,o), vec![line(f,o), line(o,p)]),
                               //  `>
                               (left.is('`'),vec![polygon(vec![f,o,p], true, vec![ArrowRight])]),
                               //  .>
                               (left.is('.'),vec![polygon(vec![f,o,p], true, vec![ArrowRight])]),
                               // >>  (double arrow)
                               (left.is('>'),  vec![polygon(vec![f,o,p], true, vec![ArrowRight])]),

                            ]
                        }
                    )
            ),
            ////////////////////////
            // less than <
            ////////////////////////
            ('<',
                vec![
                  (Medium, vec![polygon(vec![j,k,t], true, vec![ArrowLeft])]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                //    <--
                               (right.line_overlap(k,l), vec![polygon(vec![j,k,t], true, vec![ArrowLeft])]),
                               //     --<
                               (left.line_overlap(m,o) && !right.line_overlap(k,l), vec![line(j,k),line(k,t)]),
                               //  <.
                               (right.is('.'),vec![polygon(vec![j,k,t], true, vec![ArrowLeft])]),
                               //  <'
                               (right.is('\''),vec![polygon(vec![j,k,t], true, vec![ArrowLeft])]),
                               //  <<
                               (right.is('<'), vec![polygon(vec![j,k,t], true, vec![ArrowLeft])]),

                            ]
                        }
                    )
            ),
            //////////////////////
            // equal sign =
            //////////////////////
            ('=',
                vec![
                    (Medium, vec![line(_03, _43), line(_05, _45)]),
                ],
                Arc::new(
                        move|settings, top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                            vec![
                                (true, vec![line(_03, _43), line(_05, _45)]),
                            ]
                        }
                    )
            ),
        ];

        let mut btree = BTreeMap::new();
        for (ch, fragments, closure) in map{
            btree.insert(ch, Property::new(ch, fragments, closure));
        }
        btree
    };
}

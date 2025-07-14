from algo import Graph, GraphType, Position, Direction, Vector

def check_graph() -> None:
    new_graph = Graph(GraphType.OneDirectional)
    new_graph.add_edge_by_coordinates(1, 2)
    print(new_graph.get_edges())
    new_graph.remove_edge_by_coordinates(1, 2)
    print(new_graph.get_edges())

    new_graph = Graph(GraphType.BiDirectional)
    new_graph.add_edge_by_coordinates(1, 2)
    print(new_graph.get_edges())
    new_graph.remove_edge_by_coordinates(2, 1)
    print(new_graph.get_edges())

def check_position() -> None:
    position = Position(1, 2)
    position.set_new_position_for_direction(Direction.Down)
    print(position)
    position.set_new_position(Vector(-1, 0))
    print(position)
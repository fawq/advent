from algo import Graph, GraphType

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
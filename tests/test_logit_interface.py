import logit


def test_package():
    # Create database instance
    db = logit.create_db()

    # Insert a vector with a key
    db.insert("vec1", [0.1, 0.2, 0.3, 0.4])

    # Get and print the vector
    vector = db.get("vec1")
    assert [round(x, 1) for x in vector] == [0.1, 0.2, 0.3, 0.4]

    # Delete the vector
    db.delete("vec1")

    # Try to get the deleted vector
    vector = db.get("vec1")
    assert vector is None


def test_nearest_neighbor():
    # Create database instance
    db = logit.create_db()

    # Insert a vector with a key
    db.insert("vec1", [0.1, 0.2, 0.3, 0.4])
    db.insert("vec2", [-0.1, -0.2, -0.3, -0.4])

    name, vec = db.nearest_neighbor([0.2, 0.1, 0.2, 0.4])
    assert name == "vec1"
    assert vec <= 0.174

    name, vec = db.nearest_neighbor([-0.2, -0.1, -0.2, -0.4])
    assert name == "vec2"
    assert vec <= 0.1733


def test_nearest_neighbors():
    db = logit.create_db()

    # Insert a vector with a key
    db.insert("vec1", [0.1, 0.2, 0.3, 0.4])
    db.insert("vec2", [0.15, 0.25, 0.3, 0.4])
    db.insert("vec3", [-0.1, -0.2, -0.3, -0.4])

    vecs = db.nearest_neighbors([0.1, 0.2, 0.275, 0.4], 2)
    assert all(x[1] < 1.0 for x in vecs)

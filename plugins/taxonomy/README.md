# Plugin Taxonomy

This plugin provides a graph data model for representing a taxonomy within the Reactive Graph Flow. The taxonomy enables
semantic operations on the graph like graph queries and graph navigation.

## Components

| Name      | Properties | DataType | SocketType | Description                                                |
|-----------|------------|----------|------------|------------------------------------------------------------|
|           |
| weighted  | weight     | number   | none       | The weight of a relation between two entity instances      |

## Entity Types

| Name     | Components  | Properties  | DataType | SocketType | Description                     |
|----------|-------------|-------------|----------|------------|---------------------------------|
||
| category | named       | name        | string   | none       | The name of the category        |
|          | describable | description | string   | none       | The description of the category |
||
| tag      | named       | name        | string   | none       | The tag name                    |
||

## Relation Types

| Name            | Description | Components | Source Entity Type | Target Entity Type |
|-----------------|-------------|------------|--------------------|--------------------|
| categorized_as  |             |            | *                  | category           |
| has_subcategory |             |            | category           | category           |
| tagged_with     |             | weighted   | *                  | tag                |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

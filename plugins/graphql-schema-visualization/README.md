# Plugin GraphQL Schema Visualization

Visualizes the GraphQL schema.

## Overview

| URL                                                                                                               | API           | Description   |
|-------------------------------------------------------------------------------------------------------------------|---------------|---------------|
| http://localhost:31415/graphql-schema-visualization/?rootType=Query&hideRoot=false                                | GraphQL       | Queries       |
| http://localhost:31415/graphql-schema-visualization/?rootType=Mutation&hideRoot=false                             | GraphQL       | Mutations     |
| http://localhost:31415/graphql-schema-visualization/?rootType=Subscription&hideRoot=false                         | GraphQL       | Subscriptions |
| http://localhost:31415/graphql-schema-visualization/?rootType=Query&hideRoot=false&endpoint=/dynamic_graph        | Dynamic Graph | Queries       |
| http://localhost:31415/graphql-schema-visualization/?rootType=Mutation&hideRoot=false&endpoint=/dynamic_graph     | Dynamic Graph | Mutations     |
| http://localhost:31415/graphql-schema-visualization/?rootType=Subscription&hideRoot=false&endpoint=/dynamic_graph | Dynamic Graph | Subscriptions |

## GraphQL API

The GraphQL API allows low level access to the *Reactive Graph Flow*:

1. Type System
2. Instance System
3. Behaviours
4. Plugins
5. Events

## Queries

<img src="https://raw.githubusercontent.com/reactive-graph/plugins-core/main/docs/images/graphql/queries.png">

## Mutations

<img src="https://raw.githubusercontent.com/reactive-graph/plugins-core/main/docs/images/graphql/mutations.png">

## Subscriptions

<img src="https://raw.githubusercontent.com/reactive-graph/plugins-core/main/docs/images/graphql/subscriptions.png">

## Dynamic Graph API

The Dynamic Graph API enables high level access to the *Reactive Graph Flow*. It's dynamically built upon the type
system and provides type safe access to:

1. Query for instances
2. Modify instances
3. Delete instances

## Queries

<img src="https://raw.githubusercontent.com/reactive-graph/plugins-core/main/docs/images/dynamic-graph/queries.png">

## Mutations

<img src="https://raw.githubusercontent.com/reactive-graph/plugins-core/main/docs/images/dynamic-graph/mutations.png">

## Subscriptions

<img src="https://raw.githubusercontent.com/reactive-graph/plugins-core/main/docs/images/dynamic-graph/subscriptions.png">

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Credits

This plugin integrates with https://github.com/IvanGoncharov/graphql-voyager

# Mitre

Mitre is a cross-platform multi-purpose tool for running data and structural migrations on a variety of databases, data stores and similar.

It is heavily inspired by the Rails migration system, with a directory of migrations, the filenames prefixed with a timestamp which should be run once, and only once per environment.

Mitre extends this concept with orthoganal naming of the migration files (`.curl`, `.sql`, `.pgsql`, etc) which are used to look-up the corresponding runner engine and configuration.

## Filename Anatomy

```
./config/example.yml
./anylevelofnesting/example/202030303033_do_some_migration_with_our_data_models.rails
                                                                                ^^^^^ runner type, if the runner
                                                                                      needs configuration, this
                                                                                      must be provided in
                                                                                      ./config/example.yml

                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ arbitrary name for
                                                                                human readability

                            ^^^^^^^^^^^^ Datestamp for ordering across all files, is
                                         used to determine run-order for all migrations

                    ^^^^^^^ Uses the configuration in config/example.yml which may
                            contain configuration for multiple runners

  ^^^^^^^^^^^^^^^^^ Arbitrary, may also just be ./ - useful if you compose
                    your mitre migrations directory from many Git repositories
                    (see below for example)

```

## Example Directory Structure

For example:

```
config/
  ./elasticsearch.yml
  ./postgres.yml
  ./redis.yml
elasticsearch/
  202004102151_create_index.curl
  202006102151_update_index_mapping.curl
postgres/
  202030303033_create_some_table.sql
  ...
my-project/
  202030303033_do_some_migration_with_our_data_models.rails
  ...
```

In this example `.rails` is executed as a Ruby script using the `bin/rails runner` as an entrypoint using the configuration from `./config/my-project.yml`.

Various file extensions carry special meanings; `.curl` files are expected to contain command line flags to complete a `curl ...` command, e.g:

```
# cat 202004102151_create_index.curl
-X POST -d '{...giant data thing here...}'
```

## Bidirectional migrations

Mitre supports separate up-and-down migrations, by replacing the following with a directory, and two scripts, e.g :

```
rails/
  202030303033_do_some_migration_with_our_data_models.rails
  ...
```

becomes:

```
rails/
  202030303033_do_some_migration_with_our_data_models/
    up.rails
    down.rails
  ...
```

The

## Submodule friendliness

The migration directory is allowed to be nested, all files across all directories within the migration directory will be evaluated after "flattening" them and associating the relevant configuration.

This allows maintaining a Mitre set-up with migrations from a number of projects to create a kind of meta-repository that contains the migrations from a number of projects together.

## Other things to know

- config contains config files that correlate with the directories elasticsearch.yml correlates to 'elasticsearch/'

- the file extension indicates how to run the script

- a .curl extension indicates that the file in question contains params to pass to an invocation of curl, with connection params as described in the elasticsearch.yml

- across all directories things run in time order

- configuration has a concept of environments, so each of those .yml files has a `development`, `prodctuon` or whatever inside, heavily rails inspired

- You could easily do something like .risky.curl to indicate that this migration is risky, and the default mode is maybe not to run risky migrations :shrug: but you could force that

- You could support up/down migrations by making a directory `10101010101_something_reversible/{up/down}.sql`

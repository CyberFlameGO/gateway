CREATE MIGRATION m15rllnqtslgcowfv5jjpiqi5o6z7fs47eyimltvrzyeatjrob4dgq
    ONTO m13gsnvdguiuq5zvn66z2tquleart2gntbzwj3mxw2r77zayxti3qa
{
  CREATE GLOBAL default::current_edgedb_version -> std::str {
      SET default := (SELECT
          sys::get_version_as_str()
      );
  };
};

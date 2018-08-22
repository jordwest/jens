/* #comment begin
 * Templates are generally just valid example code written in the target language,
 * with annotations added to create sub-templates for later use by the compiler.
 * 
 * This means templates don't need to contain any logic and are therefore very easy
 * for humans to read. All manipulation of the template happens in Rust.
 * #comment end */
var MAP = {
    //#line-template MapEntry
    "key": "value",
};

var Logger = {
    //#template begin PrintFunction
    log_key: function () {
        console.log(MAP.key);
    },

    //#template end PrintFunction
};

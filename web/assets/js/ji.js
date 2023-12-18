$(document).ready(function() {

    $("#search").on("keyup", function() {
        let value = $(this).val().toLowerCase();
        $("#myList li").filter(function() {
            $(this).toggle($(this).text().toLowerCase().indexOf(value) > -1)
        });
    });

    $("#search-action").on("keyup", function() {
        let value = $(this).val().toLowerCase();
        $("#action li").filter(function() {
            $(this).toggle($(this).text().toLowerCase().indexOf(value) > -1)
        });
    });


    $("#show-menu").on("click", function() {
        $("#menu").toggleClass("hide").toggleClass("show");
    });

    $("#clone-repository").on("click", function(e) {
        e.preventDefault();
        let p = $("#url-to-clone").val();

        if (p == "") {
            let uri = window.location.protocol + "//" + window.location.host + "/add/";
            return window.location.replace(uri);
        }
        let uri = window.location.protocol + "//" + window.location.host + "/clone/" + p;
        return window.location.replace(uri);
    });

    $("#add-project").on("click", function(e) {
        e.preventDefault();
        let p = $("#project-new-name").val();
        let t = $("#project-type").val();
        let uri = window.location.protocol + "//" + window.location.host + "/add/" + p + "/" + t;
        return window.location.replace(uri)
    });
    $("#clean-timeline-db").on("click", function(e) {
        return confirm("Are you sure to clean the timeline ?");
    });

    $("#add-timeline").on("click", function(e) {
        e.preventDefault();
        let project = $("#project").val();
        let description = $("#timeline-description").val();
        let end = $("#timeline-end").val();
        if (description === "" || end === "") {
            let uri = window.location.protocol + '//' + window.location.host + "/time/" + project;
            return window.location.replace(uri);
        }
        let uri = window.location.protocol + "//" + window.location.host + "/add-timeline" + "/" + project + "/" + description + "/" + end;
        return window.location.replace(uri)
    });

    $(".confirm-delete").on("click", function(e) {
        let repo = $(this).attr("repo");
        return confirm("Do you really want to remove the " + repo + " directory")
    });

    $(".confirm-yank").on("click", function(e) {
        e.preventDefault();
        let repo = $(this).attr("repo");
        if (confirm("Do you really want to yank the " + repo + " repository") === true) {
            let version = prompt("please enter a version", undefined);
            let uri = window.location.protocol + '//' + window.location.host + "/yank/" + repo + "/" + version;
            return window.location.replace(uri)
        }
    });

});
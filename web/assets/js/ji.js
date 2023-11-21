$(document).ready(function () {

    $("#search").on("keyup", function () {
        let value = $(this).val().toLowerCase();
        $("#myList li").filter(function () {
            $(this).toggle($(this).text().toLowerCase().indexOf(value) > -1)
        });
    });
    $("#add-project").on("click",function (e){
       e.preventDefault();
       let p = $("#project-new-name").val();
       let t = $("#project-type").val();
       let uri = window.location.protocol + "//" + window.location.host + "/add/" + p+ "/" + t;
        return window.location.replace(uri)
    });

    $(".confirm-delete").on("click", function (e) {
        let repo = $(this).attr("repo");
        return confirm("Do you really want to remove the " + repo + " directory")
    });

    $(".confirm-yank").on("click", function (e) {
        e.preventDefault();
        let repo = $(this).attr("repo");
        if (confirm("Do you really want to yank the " + repo + " repository") === true) {
            let version = prompt("please enter a version", undefined);
            let uri = window.location.protocol + '//' + window.location.host + "/yank/"  + repo + "/" +version ;
            return window.location.replace(uri)
        }
    });

});
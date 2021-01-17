$(document).ready(function () {

    loadAndSetData();
    setInterval(loadAndSetData, 60000);
    //setInterval(reloadVisualization, 300000);

    function loadAndSetData() {
        $.getJSON("/api/v1/latest/", function (data) {
            setValues(data["internal"], data["external"], data["latest_formatted"]);
            setOWMData(data["owm_temp"], data["owm_feels"], data["owm_condition"]);
        });
    }

    function setValues(inside, outside, latest) {
        $("#temp_inside").text(inside.toFixed(1));
        $("#temp_outside").text(outside.toFixed(1));
        $("#latest").text(latest);
    }

    function setOWMData(temp, feels, condition) {
        if (parseFloat(temp) < -200.0)
            $("#owm_info").text("No Data");
        else {
            let to_show = "Current " + temp.toFixed(1) + "°C, feels like " + feels.toFixed(1) + "°C, " + condition;
            $("#owm_info").text(to_show);
        }
    }

    function reloadVisualization() {
        let d = new Date()
        $("#mgviz").attr("src", `/static/latest.png?${d.getTime()}`)
    }

});
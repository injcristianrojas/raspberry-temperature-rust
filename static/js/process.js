$(document).ready(function () {

    var myChart;

    loadAndSetData();
    setInterval(loadAndSetData, 60000);
    createChart();
    loadDataToChart();
    setInterval(loadDataToChart, 60000);

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

    function createChart() {
        var ctx = document.getElementById('myChart').getContext('2d');
        myChart = new Chart(ctx, {
            type: 'line',
            data: {
                labels: [],
                datasets: [
                    {
                        label: 'Internal',
                        data: [],
                        backgroundColor: 'cornflowerblue',
                        borderColor: 'cornflowerblue',
                        fill: false,
                    }, 
                    {
                        label: 'External',
                        data: [],
                        backgroundColor: 'orange',
                        borderColor: 'orange',
                        fill: false,
                    }
                ]
            },
            options: {
                responsive: true,
                title: {
                    display: true,
                    text: ['Chart.js Line Chart', 'Updated now'],
                    fontColor: 'white',
                    fontSize: 16,
                },
                legend: {
                    labels: {
                        fontColor: 'white',
                    }
                },
                tooltips: {
                    mode: 'index',
                    intersect: false,
                },
                hover: {
                    mode: 'nearest',
                    intersect: true
                },
                scales: {
                    xAxes: [{
                        display: true,
                        scaleLabel: {
                            display: true,
                            labelString: 'Month',
                            fontColor: 'white',
                        },
                        ticks: {
                            fontColor: 'white',
                        }
                    }],
                    yAxes: [{
                        display: true,
                        scaleLabel: {
                            display: true,
                            labelString: 'Value',
                            fontColor: 'white',
                        },
                        ticks: {
                            suggestedMin: -5,
                            suggestedMax: 40,
                            fontColor: 'white',
                        }
                    }]
                }
            }
        });
    }

    function loadDataToChart() {
        
        $.getJSON('/static/last24.json', function(response) {
            myChart.data.labels = response.labels;
            myChart.data.datasets[0].data = response.data.internal;
            myChart.data.datasets[1].data = response.data.external;
            myChart.update();
        });
    }

});
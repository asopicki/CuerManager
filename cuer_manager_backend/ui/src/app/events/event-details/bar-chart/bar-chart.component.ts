import { Component, OnInit, Input } from '@angular/core';

import { ChartOptions, ChartType, ChartDataSets, LinearTickOptions } from 'chart.js';
import { Label, Color } from 'ng2-charts';

const backgroundColors = [
  '#b71c1c',
  '#880e4f',
  '#4a148c',
  '#311b92',
  '#1a237e',
  '#0d47a1',
  '#01579b',
  '#006064',
  '#004d40',
  '#1b5e20',
  '#33691e',
  '#827717',
  '#f57f17',
  '#ff6f00',
  '#e65100'
];

const borderColors = [
  '#b71c1c',
  '#880e4f',
  '#4a148c',
  '#311b92',
  '#1a237e',
  '#0d47a1',
  '#01579b',
  '#006064',
  '#004d40',
  '#1b5e20',
  '#33691e',
  '#827717',
  '#f57f17',
  '#ff6f00',
  '#e65100'
];

@Component({
  selector: 'app-bar-chart',
  templateUrl: './bar-chart.component.html',
  styleUrls: ['./bar-chart.component.scss']
})
export class BarChartComponent implements OnInit {

  @Input() chartData: ChartDataSets[]
  @Input() chartLabels: Label[]
  @Input() chartLegend: boolean

  chartType: ChartType = 'bar'
  chartOptions: ChartOptions = {
    responsive: false,
    scales: {
      yAxes: [{
          ticks: {
            beginAtZero: true
          },
          type: 'linear'
      }]
    }
  };
  chartColors: Color[]

  constructor() { }

  ngOnInit() {
    (<LinearTickOptions>this.chartOptions.scales.yAxes[0].ticks).precision = 0;
    this.chartColors = [{
      backgroundColor: backgroundColors
    }]
  }

}

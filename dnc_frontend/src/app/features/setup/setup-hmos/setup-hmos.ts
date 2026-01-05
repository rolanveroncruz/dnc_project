import { Component } from '@angular/core';
import {GenericDataTableComponent} from "../../../components/generic-data-table-component/generic-data-table-component";
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from "@angular/material/card";

@Component({
  selector: 'app-setup-hmos',
    imports: [
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardSubtitle,
        MatCardTitle
    ],
  templateUrl: './setup-hmos.html',
  styleUrl: './setup-hmos.scss',
})
export class SetupHMOs {

}

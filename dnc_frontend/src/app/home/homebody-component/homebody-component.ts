import { Component } from '@angular/core';
import {MatButton} from "@angular/material/button";
import {
    MatCard,
    MatCardActions,
    MatCardAvatar,
    MatCardContent,
    MatCardHeader, MatCardSubtitle,
    MatCardTitle
} from "@angular/material/card";
import {MatIcon} from "@angular/material/icon";
import {MatDivider} from '@angular/material/list';
import {RouterLink} from '@angular/router';

@Component({
  selector: 'app-homebody-component',
    imports: [
        MatButton,
        MatCard,
        MatCardActions,
        MatCardAvatar,
        MatCardContent,
        MatCardHeader,
        MatCardTitle,
        MatIcon,
        MatCardSubtitle,
        MatDivider,
        RouterLink
    ],
  templateUrl: './homebody-component.html',
  styleUrl: './homebody-component.scss',
})
export class HomebodyComponent {

}

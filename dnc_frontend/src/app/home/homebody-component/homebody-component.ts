import { Component } from '@angular/core';
import {MatButton} from "@angular/material/button";
import {
    MatCardModule,
    MatCardActions,
    MatCardAvatar,
    MatCardContent,
    MatCardHeader, MatCardSubtitle,
    MatCardTitle
} from "@angular/material/card";
import {MatIcon} from "@angular/material/icon";
import {RouterLink} from '@angular/router';
import {NgOptimizedImage} from '@angular/common';

@Component({
  selector: 'app-homebody-component',
    imports: [
        MatButton,
        MatCardModule,
        MatCardActions,
        MatCardAvatar,
        MatCardContent,
        MatCardHeader,
        MatCardTitle,
        MatIcon,
        MatCardSubtitle,
        RouterLink,
        NgOptimizedImage
    ],
  templateUrl: './homebody-component.html',
  styleUrl: './homebody-component.scss',
})
export class HomebodyComponent {

}

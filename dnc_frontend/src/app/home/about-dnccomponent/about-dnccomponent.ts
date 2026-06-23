import { Component } from '@angular/core';
import {MatCard,MatCardContent, MatCardHeader, MatCardTitle} from '@angular/material/card';
import {MatIcon} from '@angular/material/icon';

@Component({
  selector: 'app-about-dnccomponent',
    imports: [
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatIcon,
        MatCardTitle
    ],
  templateUrl: './about-dnccomponent.html',
  styleUrl: './about-dnccomponent.scss',
})
export class AboutDNCComponent {

}

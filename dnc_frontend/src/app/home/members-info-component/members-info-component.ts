import { Component } from '@angular/core';
import {MatCard} from '@angular/material/card';
import {MatIcon} from '@angular/material/icon';

@Component({
  selector: 'app-members-info-component',
    imports: [
        MatCard,
        MatIcon
    ],
  templateUrl: './members-info-component.html',
  styleUrl: './members-info-component.scss',
})
export class MembersInfoComponent {

}

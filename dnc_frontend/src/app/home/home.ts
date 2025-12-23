import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatIconModule } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatButtonModule } from '@angular/material/button';
import { MatDividerModule } from '@angular/material/divider';
import {MatCardModule} from '@angular/material/card';
interface Service {
  title: string;
  description: string;
  icon: string;
}
interface Clinic {
  name: string;
  address: string;
  phone: string;
  hours: string;
}

@Component({
  selector: 'app-home',
  standalone: true,
    imports: [
      CommonModule,
      RouterModule,
      MatToolbarModule,
      MatSidenavModule,
      MatIconModule,
      MatListModule,
      MatButtonModule,
      MatCardModule,
      MatDividerModule,

    ],
  templateUrl: './home.html',
  styleUrl: './home.scss',
})
export class HomeComponent {
  services: Service[] = [
    { title: 'General Dentistry', description: 'Routine checkups, cleanings, and fillings to keep your smile healthy.', icon: 'cleaning_services' },
    { title: 'Cosmetic', description: 'Teeth whitening, veneers, and bonding for a perfect smile.', icon: 'face_retouching_natural' },
    { title: 'Orthodontics', description: 'Traditional braces and clear aligners for all ages.', icon: 'straighten' },
    { title: 'Implants & Surgery', description: 'Restorative solutions including implants and oral surgery.', icon: 'medical_services' },
  ];

  clinics: Clinic[] = [
    { name: 'Downtown Center', address: '123 Main St, Cityville', phone: '(555) 123-4567', hours: 'Mon-Fri: 8am - 6pm' },
    { name: 'Westside Family', address: '456 Oak Ave, Westtown', phone: '(555) 987-6543', hours: 'Mon-Sat: 9am - 5pm' },
    { name: 'North Hills', address: '789 Pine Rd, Northhills', phone: '(555) 321-7654', hours: 'Tue-Sat: 10am - 7pm' },
  ];

  bookAppointment() {
    console.log('Navigate to booking page');
  }

}


























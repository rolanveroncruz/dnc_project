import {Component, inject, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormControl, FormGroup, ReactiveFormsModule} from '@angular/forms';
import {MatCard} from '@angular/material/card';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {MatFormField, MatLabel, MatError} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {
    FindDentistService,
    PublicDentistSearchResult
} from './find-dentist-service';

@Component({
    selector: 'app-find-dentist',
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatCard,
        MatButton,
        MatIcon,
        MatFormField,
        MatLabel,
        MatError,
        MatInput,
        MatProgressSpinner,
    ],
    templateUrl: './find-dentist-component.html',
    styleUrl: './find-dentist-component.scss',
})
export class FindDentistComponent {
    private readonly findDentistService = inject(FindDentistService);

    readonly form = new FormGroup({
        name_query: new FormControl('', {
            nonNullable: true,
        }),
        location_query: new FormControl('', {
            nonNullable: true,
        }),
    });

    readonly results = signal<PublicDentistSearchResult[]>([]);
    readonly hasSearched = signal(false);
    readonly isSearching = signal(false);
    readonly searchError = signal<string | null>(null);
    readonly validationError = signal<string | null>(null);

    searchDentists(): void {
        const rawValue = this.form.getRawValue();

        const nameQuery = rawValue.name_query.trim();
        const locationQuery = rawValue.location_query.trim();

        if (!nameQuery && !locationQuery) {
            this.validationError.set(
                'Please enter a dentist or clinic name, address, city, or zip code.'
            );
            this.form.markAllAsTouched();
            return;
        }

        this.validationError.set(null);
        this.searchError.set(null);
        this.hasSearched.set(true);
        this.isSearching.set(true);
        this.results.set([]);

        this.findDentistService.searchDentists(nameQuery, locationQuery).subscribe({
            next: (results) => {
                this.results.set(results);
                this.isSearching.set(false);
            },
            error: (error) => {
                console.error('Error searching dentists:', error);

                this.isSearching.set(false);
                this.searchError.set(
                    'Sorry, we could not complete your search. Please try again.'
                );
            },
        });
    }

    clearSearch(): void {
        this.form.reset({
            name_query: '',
            location_query: '',
        });

        this.results.set([]);
        this.hasSearched.set(false);
        this.isSearching.set(false);
        this.searchError.set(null);
        this.validationError.set(null);
    }
}

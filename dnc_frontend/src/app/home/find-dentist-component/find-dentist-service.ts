import {inject, Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../../environments/environment';

export interface PublicDentistSearchResult {
    dentist_id: number;
    dentist_name: string;
    clinic_id: number;
    clinic_name: string;
    clinic_address: string;
    city_name: string;
    region_name: string;
    zip_code: string | null;
    contact_numbers: string | null;
    schedule: string | null;
    special_services: string[];
}

@Injectable({
    providedIn: 'root',
})
export class FindDentistService {
    private readonly http = inject(HttpClient);
    private readonly baseUrl = environment.apiUrl;

    searchDentists(nameQuery: string, locationQuery:string): Observable<PublicDentistSearchResult[]> {
        return this.http.get<PublicDentistSearchResult[]>(
            `${this.baseUrl}/public/dentists/search`,
            {
                params: {
                    name: nameQuery,
                    location: locationQuery,
                },
            }
        );
    }
}

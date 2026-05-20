import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TestChart1 } from './test-chart1';

describe('TestChart1', () => {
  let component: TestChart1;
  let fixture: ComponentFixture<TestChart1>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestChart1]
    })
    .compileComponents();

    fixture = TestBed.createComponent(TestChart1);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
